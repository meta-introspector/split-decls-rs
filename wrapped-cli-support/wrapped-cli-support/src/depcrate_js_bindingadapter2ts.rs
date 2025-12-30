// Generated macro for adapter2ts (function)
macro_rules! Depcrate_js_bindingadapter2ts {
() => {
// Module: crate::js::binding
// Provides: {"adapter2ts"}
// Dependencies: {}
fn adapter2ts (ty : & AdapterType , position : TypePosition , dst : & mut String , refs : Option < & mut HashSet < TsReference > > ,) { match ty { AdapterType :: I32 | AdapterType :: S8 | AdapterType :: S16 | AdapterType :: S32 | AdapterType :: U8 | AdapterType :: U16 | AdapterType :: U32 | AdapterType :: F32 | AdapterType :: F64 | AdapterType :: NonNull => dst . push_str ("number") , AdapterType :: I64 | AdapterType :: S64 | AdapterType :: U64 | AdapterType :: S128 | AdapterType :: U128 => dst . push_str ("bigint") , AdapterType :: String => dst . push_str ("string") , AdapterType :: Externref => dst . push_str ("any") , AdapterType :: Bool => dst . push_str ("boolean") , AdapterType :: Vector (kind) => dst . push_str (& kind . js_ty ()) , AdapterType :: Option (ty) => { adapter2ts (ty , position , dst , refs) ; dst . push_str (match position { TypePosition :: Argument => " | null | undefined" , TypePosition :: Return => " | undefined" , }) ; } AdapterType :: NamedExternref (name) => dst . push_str (name) , AdapterType :: Struct (name) => dst . push_str (name) , AdapterType :: Enum (name) => dst . push_str (name) , AdapterType :: StringEnum (name) => { if let Some (refs) = refs { refs . insert (TsReference :: StringEnum (name . clone ())) ; } dst . push_str (name) ; } AdapterType :: Function => dst . push_str ("any") , } }
};
}
