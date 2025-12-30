// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
# [doc = " *(internal)* Plucks the first mutable part."] unsafe impl < 'a , PluckedPart , Reference > PluckMut < 'a , PluckedPart , IndexHere > for Mut < PluckedPart , Reference > where PluckedPart : Part , Reference : PartialRef < 'a > , Reference :: Target : HasPart < PluckedPart > , { type Remainder = Reference ; }
};
}
