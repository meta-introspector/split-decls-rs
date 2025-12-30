// Generated macro for impl_586 (impl)
macro_rules! Depcrate_output_file_nameimpl_586 {
() => {
// Module: crate::output::file_name
// Provides: {"impl_586"}
// Dependencies: {}
impl Options { # [doc = " Create a new `FileName` that prints the given file’s name, painting it"] # [doc = " with the remaining arguments."] pub fn for_file < 'a , 'dir , C > (self , file : & 'a File < 'dir > , colours : & 'a C ,) -> FileName < 'a , 'dir , C > { FileName { file , colours , link_style : LinkStyle :: JustFilenames , options : self , target : if file . is_link () { Some (file . link_target ()) } else { None } , mount_style : MountStyle :: JustDirectoryNames , } } }
};
}
