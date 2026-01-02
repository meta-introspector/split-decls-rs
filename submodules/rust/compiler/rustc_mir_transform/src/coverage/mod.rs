mkuse!{use rustc_middle :: mir :: coverage :: { CoverageKind , FunctionCoverageInfo } ;}
mkuse!{use rustc_middle :: mir :: { self , BasicBlock , Statement , StatementKind , TerminatorKind } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use tracing :: { debug , debug_span , trace } ;}
mkuse!{use crate :: coverage :: counters :: BcbCountersData ;}
mkuse!{use crate :: coverage :: graph :: CoverageGraph ;}
mkuse!{use crate :: coverage :: mappings :: ExtractedMappings ;}
mkmod!{counters, { 
                getname!(counters);
                getsrc!(counters);
                getpath!(counters);
                get_deps!(counters);
                get_crates!(counters);
                mkinclude!(counters);
                 
            }}
mkmod!{expansion, { 
                getname!(expansion);
                getsrc!(expansion);
                getpath!(expansion);
                get_deps!(expansion);
                get_crates!(expansion);
                mkinclude!(expansion);
                 
            }}
mkmod!{graph, { 
                getname!(graph);
                getsrc!(graph);
                getpath!(graph);
                get_deps!(graph);
                get_crates!(graph);
                mkinclude!(graph);
                 
            }}
mkmod!{hir_info, { 
                getname!(hir_info);
                getsrc!(hir_info);
                getpath!(hir_info);
                get_deps!(hir_info);
                get_crates!(hir_info);
                mkinclude!(hir_info);
                 
            }}
mkmod!{mappings, { 
                getname!(mappings);
                getsrc!(mappings);
                getpath!(mappings);
                get_deps!(mappings);
                get_crates!(mappings);
                mkinclude!(mappings);
                 
            }}
mkmod!{query, { 
                getname!(query);
                getsrc!(query);
                getpath!(query);
                get_deps!(query);
                get_crates!(query);
                mkinclude!(query);
                 
            }}
mkmod!{spans, { 
                getname!(spans);
                getsrc!(spans);
                getpath!(spans);
                get_deps!(spans);
                get_crates!(spans);
                mkinclude!(spans);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkmod!{unexpand, { 
                getname!(unexpand);
                getsrc!(unexpand);
                getpath!(unexpand);
                get_deps!(unexpand);
                get_crates!(unexpand);
                mkinclude!(unexpand);
                 
            }}
mkitem!{mkstruct!{# [doc = " Inserts `StatementKind::Coverage` statements that either instrument the binary with injected"] # [doc = " counters, via intrinsic `llvm.instrprof.increment`, and/or inject metadata used during codegen"] # [doc = " to construct the coverage map."] pub (super) struct InstrumentCoverage ;}}
mkitem!{mkimpl!{impl < 'tcx > crate :: MirPass < 'tcx > for InstrumentCoverage { fn is_enabled (& self , sess : & rustc_session :: Session) -> bool { sess . instrument_coverage () } fn run_pass (& self , tcx : TyCtxt < 'tcx > , mir_body : & mut mir :: Body < 'tcx >) { let mir_source = mir_body . source ; assert ! (mir_source . promoted . is_none ()) ; let def_id = mir_source . def_id () . expect_local () ; if ! tcx . is_eligible_for_coverage (def_id) { trace ! ("InstrumentCoverage skipped for {def_id:?} (not eligible)") ; return ; } match mir_body . basic_blocks [mir :: START_BLOCK] . terminator () . kind { TerminatorKind :: Unreachable => { trace ! ("InstrumentCoverage skipped for unreachable `START_BLOCK`") ; return ; } _ => { } } instrument_function_for_coverage (tcx , mir_body) ; } fn is_required (& self) -> bool { false } }}}

macro_rules! instrument_function_for_coverage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function instrument_function_for_coverage in module {}", module_path!());
    };
}

mkfn!{
    instrument_function_for_coverage_introspect!();
    fn instrument_function_for_coverage < 'tcx > (tcx : TyCtxt < 'tcx > , mir_body : & mut mir :: Body < 'tcx >) { let def_id = mir_body . source . def_id () ; let _span = debug_span ! ("instrument_function_for_coverage" , ? def_id) . entered () ; let hir_info = hir_info :: extract_hir_info (tcx , def_id . expect_local ()) ; let graph = CoverageGraph :: from_mir (mir_body) ; let ExtractedMappings { mappings } = mappings :: extract_mappings_from_mir (tcx , mir_body , & hir_info , & graph) ; if mappings . is_empty () { debug ! ("no spans could be converted into valid mappings; skipping") ; return ; } let BcbCountersData { node_flow_data , priority_list } = counters :: prepare_bcb_counters_data (& graph) ; inject_coverage_statements (mir_body , & graph) ; mir_body . function_coverage_info = Some (Box :: new (FunctionCoverageInfo { function_source_hash : hir_info . function_source_hash , node_flow_data , priority_list , mappings , })) ; }
}

macro_rules! inject_coverage_statements_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inject_coverage_statements in module {}", module_path!());
    };
}

mkfn!{
    inject_coverage_statements_introspect!();
    # [doc = " Inject any necessary coverage statements into MIR, so that they influence codegen."] fn inject_coverage_statements < 'tcx > (mir_body : & mut mir :: Body < 'tcx > , graph : & CoverageGraph) { for (bcb , data) in graph . iter_enumerated () { let target_bb = data . leader_bb () ; inject_statement (mir_body , CoverageKind :: VirtualCounter { bcb } , target_bb) ; } }
}

macro_rules! inject_statement_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inject_statement in module {}", module_path!());
    };
}

mkfn!{
    inject_statement_introspect!();
    fn inject_statement (mir_body : & mut mir :: Body < '_ > , counter_kind : CoverageKind , bb : BasicBlock) { debug ! ("  injecting statement {counter_kind:?} for {bb:?}") ; let data = & mut mir_body [bb] ; let source_info = data . terminator () . source_info ; let statement = Statement :: new (source_info , StatementKind :: Coverage (counter_kind)) ; data . statements . insert (0 , statement) ; }
}