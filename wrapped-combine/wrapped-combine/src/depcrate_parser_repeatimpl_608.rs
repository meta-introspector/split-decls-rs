// Generated macro for impl_608 (impl)
macro_rules! Depcrate_parser_repeatimpl_608 {
() => {
// Module: crate::parser::repeat
// Provides: {"impl_608"}
// Dependencies: {}
impl < I > Iterator for SuggestSizeHint < I > where I : Iterator , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iterator . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . min , self . max) } }
};
}
