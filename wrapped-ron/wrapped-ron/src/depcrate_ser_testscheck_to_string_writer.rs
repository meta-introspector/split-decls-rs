// Generated macro for check_to_string_writer (function)
macro_rules! Depcrate_ser_testscheck_to_string_writer {
() => {
// Module: crate::ser::tests
// Provides: {"check_to_string_writer"}
// Dependencies: {}
fn check_to_string_writer < T : ? Sized + serde :: Serialize > (val : & T , check : & str , check_pretty : & str) { let ron_str = super :: to_string (val) . unwrap () ; assert_eq ! (ron_str , check) ; let ron_str_pretty = super :: to_string_pretty (val , super :: PrettyConfig :: default () . struct_names (true) . compact_structs (true) ,) . unwrap () ; assert_eq ! (ron_str_pretty , check_pretty) ; # [cfg (feature = "std")] { let mut ron_writer = std :: ffi :: OsString :: new () ; super :: to_writer (& mut ron_writer , val) . unwrap () ; assert_eq ! (ron_writer , check) ; let mut ron_writer_pretty = std :: ffi :: OsString :: new () ; super :: to_writer_pretty (& mut ron_writer_pretty , val , super :: PrettyConfig :: default () . struct_names (true) . compact_structs (true) ,) . unwrap () ; assert_eq ! (ron_writer_pretty , check_pretty) ; } }
};
}
