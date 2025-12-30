// Generated macro for assert_binary_snapshot (macro)
macro_rules! Depcrate_macrosassert_binary_snapshot {
() => {
// Module: crate::macros
// Provides: {"assert_binary_snapshot"}
// Dependencies: {}
# [doc = " (Experimental)"] # [doc = " Asserts a binary snapshot in the form of a [`Vec<u8>`]."] # [doc = ""] # [doc = " The contents get stored in a separate file next to the metadata file. The extension for this"] # [doc = " file must be passed as part of the name. For an implicit snapshot name just an extension can be"] # [doc = " passed starting with a `.`."] # [doc = ""] # [doc = " This feature is considered experimental: we may make incompatible changes for the next couple"] # [doc = " of versions after 1.41."] # [doc = ""] # [doc = " Examples:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " // implicit name:"] # [doc = " insta::assert_binary_snapshot!(\".txt\", b\"abcd\".to_vec());"] # [doc = ""] # [doc = " // named:"] # [doc = " insta::assert_binary_snapshot!(\"my_snapshot.bin\", [0, 1, 2, 3].to_vec());"] # [doc = " ```"] # [macro_export] macro_rules ! assert_binary_snapshot { ($ name_and_extension : expr , $ value : expr $ (,) ?) => { $ crate :: assert_binary_snapshot ! ($ name_and_extension , $ value , stringify ! ($ value)) ; } ; ($ name_and_extension : expr , $ value : expr , $ debug_expr : expr $ (,) ?) => { $ crate :: _macro_support :: assert_snapshot ($ crate :: _macro_support :: BinarySnapshotValue { name_and_extension : $ name_and_extension , content : $ value , } . into () , $ crate :: _get_workspace_root ! () . as_path () , $ crate :: _function_name ! () , $ crate :: _macro_support :: module_path ! () , $ crate :: _macro_support :: file ! () , $ crate :: _macro_support :: line ! () , $ debug_expr ,) . unwrap () } ; }
};
}
