// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
# [doc = " *(internal)* Plucks the first part, converting it from mutable to constant."] unsafe impl < 'a , PluckedPart , Reference > PluckConst < 'a , PluckedPart , IndexHere > for Mut < PluckedPart , Reference > where PluckedPart : Part , Reference : PartialRef < 'a > , Reference :: Target : HasPart < PluckedPart > , { type Remainder = Const < PluckedPart , Reference > ; }
};
}
