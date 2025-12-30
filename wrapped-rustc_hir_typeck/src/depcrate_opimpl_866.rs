// Generated macro for impl_866 (impl)
macro_rules! Depcrate_opimpl_866 {
() => {
// Module: crate::op
// Provides: {"impl_866"}
// Dependencies: {}
impl Op { fn span (& self) -> Span { match self { Op :: BinOp (op) => op . span , Op :: AssignOp (op) => op . span , } } fn as_str (& self) -> & 'static str { match self { Op :: BinOp (op) => op . node . as_str () , Op :: AssignOp (op) => op . node . as_str () , } } fn is_by_value (& self) -> bool { match self { Op :: BinOp (op) => op . node . is_by_value () , Op :: AssignOp (op) => op . node . is_by_value () , } } }
};
}
