// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
# [doc = " *(internal)* Skips the mutable first part while plucking a constant part."] unsafe impl < 'a , PluckedPart , SkippedPart , Reference , Index > PluckConst < 'a , PluckedPart , IndexNext < Index > > for Mut < SkippedPart , Reference > where PluckedPart : Part , SkippedPart : Part , Reference :: Target : HasPart < PluckedPart > , Reference :: Target : HasPart < SkippedPart > , Reference : PluckConst < 'a , PluckedPart , Index > , { type Remainder = Mut < SkippedPart , Reference :: Remainder > ; }
};
}
