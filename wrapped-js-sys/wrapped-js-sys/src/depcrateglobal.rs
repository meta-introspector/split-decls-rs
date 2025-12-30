// Generated macro for global (function)
macro_rules! Depcrateglobal {
() => {
// Module: crate
// Provides: {"global"}
// Dependencies: {}
# [doc = " Returns a handle to the global scope object."] # [doc = ""] # [doc = " This allows access to the global properties and global names by accessing"] # [doc = " the `Object` returned."] pub fn global () -> Object { use once_cell :: unsync :: Lazy ; struct Wrapper < T > (Lazy < T >) ; # [cfg (not (target_feature = "atomics"))] unsafe impl < T > Sync for Wrapper < T > { } # [cfg (not (target_feature = "atomics"))] unsafe impl < T > Send for Wrapper < T > { } # [cfg_attr (target_feature = "atomics" , thread_local)] static GLOBAL : Wrapper < Object > = Wrapper (Lazy :: new (get_global_object)) ; return GLOBAL . 0 . clone () ; fn get_global_object () -> Object { # [wasm_bindgen] extern "C" { type Global ; # [wasm_bindgen (thread_local_v2 , js_name = globalThis)] static GLOBAL_THIS : Option < Object > ; # [wasm_bindgen (thread_local_v2 , js_name = self)] static SELF : Option < Object > ; # [wasm_bindgen (thread_local_v2 , js_name = window)] static WINDOW : Option < Object > ; # [wasm_bindgen (thread_local_v2 , js_name = global)] static GLOBAL : Option < Object > ; } let static_object = SELF . with (Option :: clone) . or_else (| | WINDOW . with (Option :: clone)) . or_else (| | GLOBAL_THIS . with (Option :: clone)) . or_else (| | GLOBAL . with (Option :: clone)) ; if let Some (obj) = static_object { if ! obj . is_undefined () { return obj ; } } let this = Function :: new_no_args ("return this") . call0 (& JsValue :: undefined ()) . ok () ; debug_assert ! (this . is_some ()) ; match this { Some (this) => this . unchecked_into () , None => JsValue :: undefined () . unchecked_into () , } } }
};
}
