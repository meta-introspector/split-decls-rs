/* FP:mod.rs-0001 */ use crate::rustc_complete::mir::coverage::{CoverageKind, FunctionCoverageInfo};
/* FP:mod.rs-0002 */ use crate::rustc_complete::mir::{self, BasicBlock, Statement, StatementKind, TerminatorKind};
/* FP:mod.rs-0003 */ use crate::rustc_complete::ty::TyCtxt;
/* FP:mod.rs-0004 */ use tracing::{debug, debug_span, trace};
/* FP:mod.rs-0005 */ 
/* FP:mod.rs-0006 */ use crate::coverage::counters::BcbCountersData;
/* FP:mod.rs-0007 */ use crate::coverage::graph::CoverageGraph;
/* FP:mod.rs-0008 */ use crate::coverage::mappings::ExtractedMappings;
/* FP:mod.rs-0009 */ 
/* FP:mod.rs-0017 */ #[cfg(test)]
/* FP:mod.rs-0020 */ 
/* FP:mod.rs-0021 */ /// Inserts `StatementKind::Coverage` statements that either instrument the binary with injected
/* FP:mod.rs-0022 */ /// counters, via intrinsic `llvm.instrprof.increment`, and/or inject metadata used during codegen
/* FP:mod.rs-0023 */ /// to construct the coverage map.
/* FP:mod.rs-0024 */ pub(super) struct InstrumentCoverage;
/* FP:mod.rs-0025 */ 
/* FP:mod.rs-0026 */ impl<'tcx> crate::MirPass<'tcx> for InstrumentCoverage {
/* FP:mod.rs-0027 */     fn is_enabled(&self, sess: &crate::rustc_session::Session) -> bool {
/* FP:mod.rs-0028 */         sess.instrument_coverage()
/* FP:mod.rs-0029 */     }
/* FP:mod.rs-0030 */ 
/* FP:mod.rs-0031 */     fn run_pass(&self, tcx: TyCtxt<'tcx>, mir_body: &mut mir::Body<'tcx>) {
/* FP:mod.rs-0032 */         let mir_source = mir_body.source;
/* FP:mod.rs-0033 */ 
/* FP:mod.rs-0034 */         // This pass runs after MIR promotion, but before promoted MIR starts to
/* FP:mod.rs-0035 */         // be transformed, so it should never see promoted MIR.
/* FP:mod.rs-0036 */         assert!(mir_source.promoted.is_none());
/* FP:mod.rs-0037 */ 
/* FP:mod.rs-0038 */         let def_id = mir_source.def_id().expect_local();
/* FP:mod.rs-0039 */ 
/* FP:mod.rs-0040 */         if !tcx.is_eligible_for_coverage(def_id) {
/* FP:mod.rs-0041 */             trace!("InstrumentCoverage skipped for {def_id:?} (not eligible)");
/* FP:mod.rs-0042 */             return;
/* FP:mod.rs-0043 */         }
/* FP:mod.rs-0044 */ 
/* FP:mod.rs-0045 */         // An otherwise-eligible function is still skipped if its start block
/* FP:mod.rs-0046 */         // is known to be unreachable.
/* FP:mod.rs-0047 */         match mir_body.basic_blocks[mir::START_BLOCK].terminator().kind {
/* FP:mod.rs-0048 */             TerminatorKind::Unreachable => {
/* FP:mod.rs-0049 */                 trace!("InstrumentCoverage skipped for unreachable `START_BLOCK`");
/* FP:mod.rs-0050 */                 return;
/* FP:mod.rs-0051 */             }
/* FP:mod.rs-0052 */             _ => {}
/* FP:mod.rs-0053 */         }
/* FP:mod.rs-0054 */ 
/* FP:mod.rs-0055 */         instrument_function_for_coverage(tcx, mir_body);
/* FP:mod.rs-0056 */     }
/* FP:mod.rs-0057 */ 
/* FP:mod.rs-0058 */     fn is_required(&self) -> bool {
/* FP:mod.rs-0059 */         false
/* FP:mod.rs-0060 */     }
/* FP:mod.rs-0061 */ }
/* FP:mod.rs-0062 */ 
/* FP:mod.rs-0063 */ fn instrument_function_for_coverage<'tcx>(tcx: TyCtxt<'tcx>, mir_body: &mut mir::Body<'tcx>) {
/* FP:mod.rs-0064 */     let def_id = mir_body.source.def_id();
/* FP:mod.rs-0065 */     let _span = debug_span!("instrument_function_for_coverage", ?def_id).entered();
/* FP:mod.rs-0066 */ 
/* FP:mod.rs-0067 */     let hir_info = hir_info::extract_hir_info(tcx, def_id.expect_local());
/* FP:mod.rs-0068 */ 
/* FP:mod.rs-0069 */     // Build the coverage graph, which is a simplified view of the MIR control-flow
/* FP:mod.rs-0070 */     // graph that ignores some details not relevant to coverage instrumentation.
/* FP:mod.rs-0071 */     let graph = CoverageGraph::from_mir(mir_body);
/* FP:mod.rs-0072 */ 
/* FP:mod.rs-0073 */     ////////////////////////////////////////////////////
/* FP:mod.rs-0074 */     // Extract coverage spans and other mapping info from MIR.
/* FP:mod.rs-0075 */     let ExtractedMappings { mappings } =
/* FP:mod.rs-0076 */         mappings::extract_mappings_from_mir(tcx, mir_body, &hir_info, &graph);
/* FP:mod.rs-0077 */     if mappings.is_empty() {
/* FP:mod.rs-0078 */         // No spans could be converted into valid mappings, so skip this function.
/* FP:mod.rs-0079 */         debug!("no spans could be converted into valid mappings; skipping");
/* FP:mod.rs-0080 */         return;
/* FP:mod.rs-0081 */     }
/* FP:mod.rs-0082 */ 
/* FP:mod.rs-0083 */     // Use the coverage graph to prepare intermediate data that will eventually
/* FP:mod.rs-0084 */     // be used to assign physical counters and counter expressions to points in
/* FP:mod.rs-0085 */     // the control-flow graph.
/* FP:mod.rs-0086 */     let BcbCountersData { node_flow_data, priority_list } =
/* FP:mod.rs-0087 */         counters::prepare_bcb_counters_data(&graph);
/* FP:mod.rs-0088 */ 
/* FP:mod.rs-0089 */     // Inject coverage statements into MIR.
/* FP:mod.rs-0090 */     inject_coverage_statements(mir_body, &graph);
/* FP:mod.rs-0091 */ 
/* FP:mod.rs-0092 */     mir_body.function_coverage_info = Some(Box::new(FunctionCoverageInfo {
/* FP:mod.rs-0093 */         function_source_hash: hir_info.function_source_hash,
/* FP:mod.rs-0094 */ 
/* FP:mod.rs-0095 */         node_flow_data,
/* FP:mod.rs-0096 */         priority_list,
/* FP:mod.rs-0097 */ 
/* FP:mod.rs-0098 */         mappings,
/* FP:mod.rs-0099 */     }));
/* FP:mod.rs-0100 */ }
/* FP:mod.rs-0101 */ 
/* FP:mod.rs-0102 */ /// Inject any necessary coverage statements into MIR, so that they influence codegen.
/* FP:mod.rs-0103 */ fn inject_coverage_statements<'tcx>(mir_body: &mut mir::Body<'tcx>, graph: &CoverageGraph) {
/* FP:mod.rs-0104 */     for (bcb, data) in graph.iter_enumerated() {
/* FP:mod.rs-0105 */         let target_bb = data.leader_bb();
/* FP:mod.rs-0106 */         inject_statement(mir_body, CoverageKind::VirtualCounter { bcb }, target_bb);
/* FP:mod.rs-0107 */     }
/* FP:mod.rs-0108 */ }
/* FP:mod.rs-0109 */ 
/* FP:mod.rs-0110 */ fn inject_statement(mir_body: &mut mir::Body<'_>, counter_kind: CoverageKind, bb: BasicBlock) {
/* FP:mod.rs-0111 */     debug!("  injecting statement {counter_kind:?} for {bb:?}");
/* FP:mod.rs-0112 */     let data = &mut mir_body[bb];
/* FP:mod.rs-0113 */     let source_info = data.terminator().source_info;
/* FP:mod.rs-0114 */     let statement = Statement::new(source_info, StatementKind::Coverage(counter_kind));
/* FP:mod.rs-0115 */     data.statements.insert(0, statement);
/* FP:mod.rs-0116 */ }