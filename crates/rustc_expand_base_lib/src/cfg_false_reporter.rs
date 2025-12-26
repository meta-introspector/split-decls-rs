// submodules/rust/compiler/rustc_expand_base_lib/src/cfg_false_reporter.rs
use rustc_span::Span;
use rustc_ast::node_id::NodeId;
use rustc_ast::{HasAttrs, HasNodeId};

pub trait CfgFalseReporter {
    // Reports a #[cfg(FALSE)] situation, handling linting and error buffering.
    // This method is called from InvocationCollector::visit_node.
    fn report_cfg_false<N: HasAttrs + HasNodeId>(&mut self, node: &mut N, attr_span: Span, attr_pos: usize);
}
