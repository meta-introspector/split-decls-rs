// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
# [doc = " *(internal)* Plucks the outermost constant part."] unsafe impl < 'a , PluckedPart , Reference > PluckConst < 'a , PluckedPart , IndexHere > for Const < PluckedPart , Reference > where PluckedPart : Part , Reference : PartialRef < 'a > , Reference :: Target : HasPart < PluckedPart > , { type Remainder = Const < PluckedPart , Reference > ; }
};
}
