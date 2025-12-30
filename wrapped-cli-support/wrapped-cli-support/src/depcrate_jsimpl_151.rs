// Generated macro for impl_151 (impl)
macro_rules! Depcrate_jsimpl_151 {
() => {
// Module: crate::js
// Provides: {"impl_151"}
// Dependencies: {}
impl ExportedClass { fn push (& mut self , function_name : & str , function_prefix : & str , js_docs : & str , js : & str , ts_docs : & str , ts : Option < & str > ,) { self . contents . push_str (js_docs) ; self . contents . push_str (function_prefix) ; self . contents . push_str (function_name) ; self . contents . push_str (js) ; self . contents . push ('\n') ; if let Some (ts) = ts { if ! ts_docs . is_empty () { for line in ts_docs . lines () { self . typescript . push_str ("  ") ; self . typescript . push_str (line) ; self . typescript . push ('\n') ; } } self . typescript . push_str ("  ") ; self . typescript . push_str (function_prefix) ; self . typescript . push_str (function_name) ; self . typescript . push_str (ts) ; self . typescript . push_str (";\n") ; } } fn push_accessor_ts (& mut self , location : FieldLocation , accessor : FieldAccessor , is_setter : bool ,) { let size = self . typescript_fields . len () ; let field = self . typescript_fields . entry (location) . or_insert_with_key (| location | FieldInfo { name : location . name . to_string () , is_static : location . is_static , order : size , getter : None , setter : None , }) ; if is_setter { field . setter = Some (accessor) ; } else { field . getter = Some (accessor) ; } } }
};
}
