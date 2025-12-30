// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
# [doc = " *(internal)* Every reference has the empty reference as subset."] unsafe impl < 'a , Reference > HasSubset < 'a , Ref < 'a , Reference :: Target > , SubsetIndexEnd > for Reference where Reference : PartialRef < 'a > , { type Remainder = Reference ; }
};
}
