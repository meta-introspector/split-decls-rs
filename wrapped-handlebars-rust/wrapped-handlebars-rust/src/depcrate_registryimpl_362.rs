// Generated macro for impl_362 (impl)
macro_rules! Depcrate_registryimpl_362 {
() => {
// Module: crate::registry
// Provides: {"impl_362"}
// Dependencies: {}
# [cfg (feature = "dir_source")] impl DirectorySourceOptions { fn ignore_file (& self , name : & str) -> bool { self . ignored_as_hidden_file (name) || self . ignored_as_temporary_file (name) } # [inline] fn ignored_as_hidden_file (& self , name : & str) -> bool { ! self . hidden && name . starts_with ('.') } # [inline] fn ignored_as_temporary_file (& self , name : & str) -> bool { ! self . temporary && name . starts_with ('#') } }
};
}
