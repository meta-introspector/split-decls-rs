/* FP:expansion.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_expansion_UNPARSEABLE_0001
/* FP:expansion.rs-0002 */ use crate::rustc_data_structures::fx::{FxIndexMap, FxIndexSet, IndexEntry};
/* FP:expansion.rs-0003 */ use crate::rustc_complete::mir::coverage::BasicCoverageBlock;
/* FP:expansion.rs-0004 */ use crate::rustc_complete::{ExpnId, ExpnKind, Span};
/* FP:expansion.rs-0005 */ 
/* FP:expansion.rs-0006 */ #[derive(Clone, Copy, Debug)]
/* FP:expansion.rs-0007 */ pub(crate) struct SpanWithBcb {
/* FP:expansion.rs-0008 */     pub(crate) span: Span,
/* FP:expansion.rs-0009 */     pub(crate) bcb: BasicCoverageBlock,
/* FP:expansion.rs-0010 */ }
/* FP:expansion.rs-0011 */ 
/* FP:expansion.rs-0012 */ #[derive(Debug)]
/* FP:expansion.rs-0013 */ pub(crate) struct ExpnTree {
/* FP:expansion.rs-0014 */     nodes: FxIndexMap<ExpnId, ExpnNode>,
/* FP:expansion.rs-0015 */ }
/* FP:expansion.rs-0016 */ 
/* FP:expansion.rs-0017 */ impl ExpnTree {
/* FP:expansion.rs-0018 */     pub(crate) fn get(&self, expn_id: ExpnId) -> Option<&ExpnNode> {
/* FP:expansion.rs-0019 */         self.nodes.get(&expn_id)
/* FP:expansion.rs-0020 */     }
/* FP:expansion.rs-0021 */ 
/* FP:expansion.rs-0022 */     /// Yields the tree node for the given expansion ID (if present), followed
/* FP:expansion.rs-0023 */     /// by the nodes of all of its descendants in depth-first order.
/* FP:expansion.rs-0024 */     pub(crate) fn iter_node_and_descendants(
/* FP:expansion.rs-0025 */         &self,
/* FP:expansion.rs-0026 */         root_expn_id: ExpnId,
/* FP:expansion.rs-0027 */     ) -> impl Iterator<Item = &ExpnNode> {
/* FP:expansion.rs-0028 */         gen move {
/* FP:expansion.rs-0029 */             let Some(root_node) = self.get(root_expn_id) else { return };
/* FP:expansion.rs-0030 */             yield root_node;
/* FP:expansion.rs-0031 */ 
/* FP:expansion.rs-0032 */             // Stack of child-node-ID iterators that drives the depth-first traversal.
/* FP:expansion.rs-0033 */             let mut iter_stack = vec![root_node.child_expn_ids.iter()];
/* FP:expansion.rs-0034 */ 
/* FP:expansion.rs-0035 */             while let Some(curr_iter) = iter_stack.last_mut() {
/* FP:expansion.rs-0036 */                 // Pull the next ID from the top of the stack.
/* FP:expansion.rs-0037 */                 let Some(&curr_id) = curr_iter.next() else {
/* FP:expansion.rs-0038 */                     iter_stack.pop();
/* FP:expansion.rs-0039 */                     continue;
/* FP:expansion.rs-0040 */                 };
/* FP:expansion.rs-0041 */ 
/* FP:expansion.rs-0042 */                 // Yield this node.
/* FP:expansion.rs-0043 */                 let Some(node) = self.get(curr_id) else { continue };
/* FP:expansion.rs-0044 */                 yield node;
/* FP:expansion.rs-0045 */ 
/* FP:expansion.rs-0046 */                 // Push the node's children, to be traversed next.
/* FP:expansion.rs-0047 */                 if !node.child_expn_ids.is_empty() {
/* FP:expansion.rs-0048 */                     iter_stack.push(node.child_expn_ids.iter());
/* FP:expansion.rs-0049 */                 }
/* FP:expansion.rs-0050 */             }
/* FP:expansion.rs-0051 */         }
/* FP:expansion.rs-0052 */     }
/* FP:expansion.rs-0053 */ }
/* FP:expansion.rs-0054 */ 
/* FP:expansion.rs-0055 */ #[derive(Debug)]
/* FP:expansion.rs-0056 */ pub(crate) struct ExpnNode {
/* FP:expansion.rs-0057 */     /// Storing the expansion ID in its own node is not strictly necessary,
/* FP:expansion.rs-0058 */     /// but is helpful for debugging and might be useful later.
/* FP:expansion.rs-0059 */     #[expect(dead_code)]
/* FP:expansion.rs-0060 */     pub(crate) expn_id: ExpnId,
/* FP:expansion.rs-0061 */ 
/* FP:expansion.rs-0062 */     // Useful info extracted from `ExpnData`.
/* FP:expansion.rs-0063 */     pub(crate) expn_kind: ExpnKind,
/* FP:expansion.rs-0064 */     /// Non-dummy `ExpnData::call_site` span.
/* FP:expansion.rs-0065 */     pub(crate) call_site: Option<Span>,
/* FP:expansion.rs-0066 */     /// Expansion ID of `call_site`, if present.
/* FP:expansion.rs-0067 */     /// This links an expansion node to its parent in the tree.
/* FP:expansion.rs-0068 */     pub(crate) call_site_expn_id: Option<ExpnId>,
/* FP:expansion.rs-0069 */ 
/* FP:expansion.rs-0070 */     /// Spans (and their associated BCBs) belonging to this expansion.
/* FP:expansion.rs-0071 */     pub(crate) spans: Vec<SpanWithBcb>,
/* FP:expansion.rs-0072 */     /// Expansions whose call-site is in this expansion.
/* FP:expansion.rs-0073 */     pub(crate) child_expn_ids: FxIndexSet<ExpnId>,
/* FP:expansion.rs-0074 */ }
/* FP:expansion.rs-0075 */ 
/* FP:expansion.rs-0076 */ impl ExpnNode {
/* FP:expansion.rs-0077 */     fn new(expn_id: ExpnId) -> Self {
/* FP:expansion.rs-0078 */         let expn_data = expn_id.expn_data();
/* FP:expansion.rs-0079 */ 
/* FP:expansion.rs-0080 */         let call_site = Some(expn_data.call_site).filter(|sp| !sp.is_dummy());
/* FP:expansion.rs-0081 */         let call_site_expn_id = try { call_site?.ctxt().outer_expn() };
/* FP:expansion.rs-0082 */ 
/* FP:expansion.rs-0083 */         Self {
/* FP:expansion.rs-0084 */             expn_id,
/* FP:expansion.rs-0085 */ 
/* FP:expansion.rs-0086 */             expn_kind: expn_data.kind,
/* FP:expansion.rs-0087 */             call_site,
/* FP:expansion.rs-0088 */             call_site_expn_id,
/* FP:expansion.rs-0089 */ 
/* FP:expansion.rs-0090 */             spans: vec![],
/* FP:expansion.rs-0091 */             child_expn_ids: FxIndexSet::default(),
/* FP:expansion.rs-0092 */         }
/* FP:expansion.rs-0093 */     }
/* FP:expansion.rs-0094 */ }
/* FP:expansion.rs-0095 */ 
/* FP:expansion.rs-0096 */ /// Given a collection of span/BCB pairs from potentially-different syntax contexts,
/* FP:expansion.rs-0097 */ /// arranges them into an "expansion tree" based on their expansion call-sites.
/* FP:expansion.rs-0098 */ pub(crate) fn build_expn_tree(spans: impl IntoIterator<Item = SpanWithBcb>) -> ExpnTree {
/* FP:expansion.rs-0099 */     let mut nodes = FxIndexMap::default();
/* FP:expansion.rs-0100 */     let new_node = |&expn_id: &ExpnId| ExpnNode::new(expn_id);
/* FP:expansion.rs-0101 */ 
/* FP:expansion.rs-0102 */     for span_with_bcb in spans {
/* FP:expansion.rs-0103 */         // Create a node for this span's enclosing expansion, and add the span to it.
/* FP:expansion.rs-0104 */         let expn_id = span_with_bcb.span.ctxt().outer_expn();
/* FP:expansion.rs-0105 */         let node = nodes.entry(expn_id).or_insert_with_key(new_node);
/* FP:expansion.rs-0106 */         node.spans.push(span_with_bcb);
/* FP:expansion.rs-0107 */ 
/* FP:expansion.rs-0108 */         // Now walk up the expansion call-site chain, creating nodes and registering children.
/* FP:expansion.rs-0109 */         let mut prev = expn_id;
/* FP:expansion.rs-0110 */         let mut curr_expn_id = node.call_site_expn_id;
/* FP:expansion.rs-0111 */         while let Some(expn_id) = curr_expn_id {
/* FP:expansion.rs-0112 */             let entry = nodes.entry(expn_id);
/* FP:expansion.rs-0113 */             let node_existed = matches!(entry, IndexEntry::Occupied(_));
/* FP:expansion.rs-0114 */ 
/* FP:expansion.rs-0115 */             let node = entry.or_insert_with_key(new_node);
/* FP:expansion.rs-0116 */             node.child_expn_ids.insert(prev);
/* FP:expansion.rs-0117 */ 
/* FP:expansion.rs-0118 */             if node_existed {
/* FP:expansion.rs-0119 */                 break;
/* FP:expansion.rs-0120 */             }
/* FP:expansion.rs-0121 */ 
/* FP:expansion.rs-0122 */             prev = expn_id;
/* FP:expansion.rs-0123 */             curr_expn_id = node.call_site_expn_id;
/* FP:expansion.rs-0124 */         }
/* FP:expansion.rs-0125 */     }
/* FP:expansion.rs-0126 */ 
/* FP:expansion.rs-0127 */     ExpnTree { nodes }
/* FP:expansion.rs-0128 */ }