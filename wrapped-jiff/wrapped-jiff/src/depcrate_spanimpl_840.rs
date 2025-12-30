// Generated macro for impl_840 (impl)
macro_rules! Depcrate_spanimpl_840 {
() => {
// Module: crate::span
// Provides: {"impl_840"}
// Dependencies: {}
impl From < Date > for SpanRelativeTo < 'static > { fn from (date : Date) -> SpanRelativeTo < 'static > { let dt = DateTime :: from_parts (date , Time :: midnight ()) ; SpanRelativeTo { kind : SpanRelativeToKind :: Civil (dt) } } }
};
}
