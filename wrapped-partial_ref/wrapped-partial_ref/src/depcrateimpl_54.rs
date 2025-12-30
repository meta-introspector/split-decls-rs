// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
# [doc = " *(internal)* Skips the mutable first part while plucking a mutable part."] unsafe impl < 'a , PluckedPart , SkippedPart , Reference , Index > PluckMut < 'a , PluckedPart , IndexNext < Index > > for Mut < SkippedPart , Reference > where PluckedPart : Part , SkippedPart : Part , Reference :: Target : HasPart < PluckedPart > , Reference :: Target : HasPart < SkippedPart > , Reference : PluckMut < 'a , PluckedPart , Index > , { type Remainder = Mut < SkippedPart , Reference :: Remainder > ; }
};
}
