// Generated macro for impl_22 (impl)
macro_rules! Depcrate_rootimpl_22 {
() => {
// Module: crate::root
// Provides: {"impl_22"}
// Dependencies: {}
impl DirEntry { # [doc = " Returns the file name of this directory entry without any"] # [doc = " leading path component(s)."] # [doc = ""] # [doc = " As an example,"] # [doc = " the output of the function will result in \"foo\" for all the following paths:"] # [doc = " - \"./foo\""] # [doc = " - \"/the/foo\""] # [doc = " - \"../../foo\""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use relative_path_utils::Root;"] # [doc = ""] # [doc = " let mut root = Root::new(\".\")?;"] # [doc = ""] # [doc = " for entry in root.read_dir(\"src\")? {"] # [doc = "     let entry = entry?;"] # [doc = "     println!(\"{:?}\", entry.file_name());"] # [doc = " }"] # [doc = " # Ok::<_, std::io::Error>(())"] # [doc = " ```"] # [must_use] pub fn file_name (& self) -> OsString { self . inner . file_name () } }
};
}
