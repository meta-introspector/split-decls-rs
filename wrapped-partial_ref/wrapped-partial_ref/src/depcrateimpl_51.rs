// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
# [doc = " *(internal)* Skips the constant first part while plucking a constant part."] unsafe impl < 'a , PluckedPart , SkippedPart , Reference , Index > PluckConst < 'a , PluckedPart , IndexNext < Index > > for Const < SkippedPart , Reference > where PluckedPart : Part , SkippedPart : Part , Reference :: Target : HasPart < PluckedPart > , Reference :: Target : HasPart < SkippedPart > , Reference : PluckConst < 'a , PluckedPart , Index > , { type Remainder = Const < SkippedPart , Reference :: Remainder > ; }
};
}
