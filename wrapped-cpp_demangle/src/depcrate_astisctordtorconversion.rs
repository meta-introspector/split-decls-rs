// Generated macro for IsCtorDtorConversion (trait)
macro_rules! Depcrate_astIsCtorDtorConversion {
() => {
// Module: crate::ast
// Provides: {"IsCtorDtorConversion"}
// Dependencies: {}
# [doc = " Determine whether this AST node is a constructor, destructor, or conversion"] # [doc = " function."] pub (crate) trait IsCtorDtorConversion { fn is_ctor_dtor_conversion (& self , subs : & SubstitutionTable) -> bool ; }
};
}
