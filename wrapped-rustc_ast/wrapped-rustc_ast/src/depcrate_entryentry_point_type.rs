// Generated macro for entry_point_type (function)
macro_rules! Depcrate_entryentry_point_type {
() => {
// Module: crate::entry
// Provides: {"entry_point_type"}
// Dependencies: {}
pub fn entry_point_type (attrs : & [impl AttributeExt] , at_root : bool , name : Option < Symbol > ,) -> EntryPointType { if attr :: contains_name (attrs , sym :: rustc_main) { EntryPointType :: RustcMainAttr } else if let Some (name) = name && name == sym :: main { if at_root { EntryPointType :: MainNamed } else { EntryPointType :: OtherMain } } else { EntryPointType :: None } }
};
}
