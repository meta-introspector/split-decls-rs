// Generated macro for PathElem (enum)
macro_rules! Depcrate_interpret_validityPathElem {
() => {
// Module: crate::interpret::validity
// Provides: {"PathElem"}
// Dependencies: {}
# [doc = " We want to show a nice path to the invalid field for diagnostics,"] # [doc = " but avoid string operations in the happy case where no error happens."] # [doc = " So we track a `Vec<PathElem>` where `PathElem` contains all the data we"] # [doc = " need to later print something for the user."] # [derive (Copy , Clone , Debug)] pub enum PathElem { Field (Symbol) , Variant (Symbol) , CoroutineState (VariantIdx) , CapturedVar (Symbol) , ArrayElem (usize) , TupleElem (usize) , Deref , EnumTag , CoroutineTag , DynDowncast , Vtable , }
};
}
