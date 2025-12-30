// Generated macro for get_stack_pointer (function)
macro_rules! Depcrate_wasm_conventionsget_stack_pointer {
() => {
// Module: crate::wasm_conventions
// Provides: {"get_stack_pointer"}
// Dependencies: {}
# [doc = " Get the `__stack_pointer`."] pub fn get_stack_pointer (module : & Module) -> Option < GlobalId > { if let Some (g) = module . globals . iter () . find (| g | matches ! (g . name . as_deref () , Some ("__stack_pointer"))) { return Some (g . id ()) ; } let candidates = module . globals . iter () . filter (| g | g . ty == ValType :: I32) . filter (| g | g . mutable) . filter (| g | match g . kind { GlobalKind :: Local (ConstExpr :: Value (Value :: I32 (n))) => n != 0 , _ => false , }) . collect :: < Vec < _ > > () ; match candidates . len () { 0 => None , 1 => Some (candidates [0] . id ()) , 2 => { log :: warn ! ("Unable to accurately determine the location of `__stack_pointer`") ; Some (candidates [0] . id ()) } _ => None , } }
};
}
