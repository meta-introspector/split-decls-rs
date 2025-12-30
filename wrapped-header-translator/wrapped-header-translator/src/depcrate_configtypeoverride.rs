// Generated macro for TypeOverride (struct)
macro_rules! Depcrate_configTypeOverride {
() => {
// Module: crate::config
// Provides: {"TypeOverride"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default , Clone , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub struct TypeOverride { # [serde (default)] pub nullability : Option < Nullability > , # [serde (default)] pub generics : Option < Vec < ItemGeneric > > , # [serde (default)] pub bounds : PointerBounds , # [doc = " Override whether this pointer is read from."] # [doc = ""] # [doc = " By default, this is assumed for all pointers, but if we set this to"] # [doc = " `false`, we can generate `MaybeUninit<T>` instead."] # [serde (default)] pub read : Option < bool > , # [doc = " Override whether this pointer is written to."] # [doc = ""] # [doc = " By default, this is the inverse of the `const`-ness of the pointer."] # [doc = " That is safe by default, but in many cases we want to set this to"] # [doc = " `false` to generate `*const T`/`&T` instead of `*mut T`/`&mut T`."] # [serde (default)] pub written : Option < bool > , }
};
}
