// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
# [doc = " *(internal)* Skips the constant first part while plucking a mutable part."] unsafe impl < 'a , PluckedPart , SkippedPart , Reference , Index > PluckMut < 'a , PluckedPart , IndexNext < Index > > for Const < SkippedPart , Reference > where PluckedPart : Part , SkippedPart : Part , Reference :: Target : HasPart < PluckedPart > , Reference :: Target : HasPart < SkippedPart > , Reference : PluckMut < 'a , PluckedPart , Index > , { type Remainder = Const < SkippedPart , Reference :: Remainder > ; }
};
}
