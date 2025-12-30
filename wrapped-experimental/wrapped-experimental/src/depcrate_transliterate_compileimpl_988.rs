// Generated macro for impl_988 (impl)
macro_rules! Depcrate_transliterate_compileimpl_988 {
() => {
// Module: crate::transliterate::compile
// Provides: {"impl_988"}
// Dependencies: {}
impl Direction { # [doc = " Whether `self` is a superset of `other` or not."] pub (crate) fn permits (self , other : Direction) -> bool { match self { Direction :: Forward => other == Direction :: Forward , Direction :: Reverse => other == Direction :: Reverse , Direction :: Both => true , } } }
};
}
