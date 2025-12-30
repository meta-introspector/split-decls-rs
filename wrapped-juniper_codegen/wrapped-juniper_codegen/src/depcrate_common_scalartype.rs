// Generated macro for Type (enum)
macro_rules! Depcrate_common_scalarType {
() => {
// Module: crate::common::scalar
// Provides: {"Type"}
// Dependencies: {}
# [doc = " [`ScalarValue`] parametrization of the code generation."] # [doc = ""] # [doc = " [`ScalarValue`]: juniper::ScalarValue"] # [derive (Clone , Debug)] pub (crate) enum Type { # [doc = " Concrete Rust type is specified as [`ScalarValue`]."] # [doc = ""] # [doc = " [`ScalarValue`]: juniper::ScalarValue"] Concrete (syn :: Type) , # [doc = " One of type parameters of the original type is specified as [`ScalarValue`]."] # [doc = ""] # [doc = " The original type is the type that the code is generated for."] # [doc = ""] # [doc = " [`ScalarValue`]: juniper::ScalarValue"] ExplicitGeneric (syn :: Ident) , # [doc = " [`ScalarValue`] parametrization is assumed to be generic and is not specified"] # [doc = " explicitly, or specified as bound predicate (like `S: ScalarValue + Send + Sync`)."] # [doc = ""] # [doc = " [`ScalarValue`]: juniper::ScalarValue"] ImplicitGeneric (Option < syn :: PredicateType >) , }
};
}
