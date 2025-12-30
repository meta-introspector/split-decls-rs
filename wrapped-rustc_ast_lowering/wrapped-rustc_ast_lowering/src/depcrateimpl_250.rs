// Generated macro for impl_250 (impl)
macro_rules! Depcrateimpl_250 {
() => {
// Module: crate
// Provides: {"impl_250"}
// Dependencies: {}
impl SpanLowerer { fn lower (& self , span : Span) -> Span { if self . is_incremental { span . with_parent (Some (self . def_id)) } else { span } } }
};
}
