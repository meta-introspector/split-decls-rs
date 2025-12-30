// Generated macro for Nested (struct)
macro_rules! DepcrateNested {
() => {
// Module: crate
// Provides: {"Nested"}
// Dependencies: {}
# [doc = " A nested part."] # [doc = ""] # [doc = " A nested part can be constructed from an outer part and an inner part. The outer part must be a"] # [doc = " [`Field`] part, and the field's type must be a [`PartialRefTarget`] having the the inner part"] # [doc = " ([`HasPart`])."] # [doc = ""] # [doc = " When nesting multiple times, the nested part should always be the outer part. This isn't"] # [doc = " enforced, but some operations are only supported in that case."] # [derive (Default)] pub struct Nested < Outer , Inner > (Outer , Inner) ;
};
}
