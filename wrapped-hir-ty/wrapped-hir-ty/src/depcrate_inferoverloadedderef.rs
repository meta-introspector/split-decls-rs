// Generated macro for OverloadedDeref (struct)
macro_rules! Depcrate_inferOverloadedDeref {
() => {
// Module: crate::infer
// Provides: {"OverloadedDeref"}
// Dependencies: {}
# [doc = " An overloaded autoderef step, representing a `Deref(Mut)::deref(_mut)`"] # [doc = " call, with the signature `&'a T -> &'a U` or `&'a mut T -> &'a mut U`."] # [doc = " The target type is `U` in both cases, with the region and mutability"] # [doc = " being those shared by both the receiver and the returned reference."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct OverloadedDeref (pub Option < Mutability >) ;
};
}
