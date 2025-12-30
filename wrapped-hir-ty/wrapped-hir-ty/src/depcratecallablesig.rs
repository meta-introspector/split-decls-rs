// Generated macro for CallableSig (struct)
macro_rules! DepcrateCallableSig {
() => {
// Module: crate
// Provides: {"CallableSig"}
// Dependencies: {}
# [doc = " A function signature as seen by type inference: Several parameter types and"] # [doc = " one return type."] # [derive (Clone , PartialEq , Eq , Debug)] pub struct CallableSig { params_and_return : Arc < [Ty] > , is_varargs : bool , safety : Safety , abi : FnAbi , }
};
}
