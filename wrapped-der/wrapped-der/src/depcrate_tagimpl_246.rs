// Generated macro for impl_246 (impl)
macro_rules! Depcrate_tagimpl_246 {
() => {
// Module: crate::tag
// Provides: {"impl_246"}
// Dependencies: {}
# [doc = " Types which are [`FixedTag`] always known if they are constructed (or primitive)."] impl < T : FixedTag + ? Sized > IsConstructed for T { const CONSTRUCTED : bool = T :: TAG . is_constructed () ; }
};
}
