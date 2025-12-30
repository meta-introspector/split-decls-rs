// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
# [doc = " Create default values for nested parts."] # [doc = ""] # [doc = " These are useful as parameters to pass to [`PartialRef`]'s part functions."] impl < NewInnerPart : Part , Outer : Part , Inner : Part > std :: ops :: BitOr < NewInnerPart > for Nested < Outer , Inner > { type Output = Nested < Nested < Outer , Inner > , NewInnerPart > ; fn bitor (self , _rhs : NewInnerPart) -> Self :: Output { std :: default :: Default :: default () } }
};
}
