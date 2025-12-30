// Generated macro for write_path (function)
macro_rules! Depcrate_interpret_validitywrite_path {
() => {
// Module: crate::interpret::validity
// Provides: {"write_path"}
// Dependencies: {}
# [doc = " Format a path"] fn write_path (out : & mut String , path : & [PathElem]) { use self :: PathElem :: * ; for elem in path . iter () { match elem { Field (name) => write ! (out , ".{name}") , EnumTag => write ! (out , ".<enum-tag>") , Variant (name) => write ! (out , ".<enum-variant({name})>") , CoroutineTag => write ! (out , ".<coroutine-tag>") , CoroutineState (idx) => write ! (out , ".<coroutine-state({})>" , idx . index ()) , CapturedVar (name) => write ! (out , ".<captured-var({name})>") , TupleElem (idx) => write ! (out , ".{idx}") , ArrayElem (idx) => write ! (out , "[{idx}]") , Deref => write ! (out , ".<deref>") , DynDowncast => write ! (out , ".<dyn-downcast>") , Vtable => write ! (out , ".<vtable>") , } . unwrap () } }
};
}
