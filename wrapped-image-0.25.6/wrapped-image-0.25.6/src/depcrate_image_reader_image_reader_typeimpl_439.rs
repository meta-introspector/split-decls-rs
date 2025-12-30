// Generated macro for impl_439 (impl)
macro_rules! Depcrate_image_reader_image_reader_typeimpl_439 {
() => {
// Module: crate::image_reader::image_reader_type
// Provides: {"impl_439"}
// Dependencies: {}
impl ImageReader < BufReader < File > > { # [doc = " Open a file to read, format will be guessed from path."] # [doc = ""] # [doc = " This will not attempt any io operation on the opened file."] # [doc = ""] # [doc = " If you want to inspect the content for a better guess on the format, which does not depend"] # [doc = " on file extensions, follow this call with a call to [`with_guessed_format`]."] # [doc = ""] # [doc = " [`with_guessed_format`]: #method.with_guessed_format"] pub fn open < P > (path : P) -> io :: Result < Self > where P : AsRef < Path > , { Self :: open_impl (path . as_ref ()) } fn open_impl (path : & Path) -> io :: Result < Self > { Ok (ImageReader { inner : BufReader :: new (File :: open (path) ?) , format : ImageFormat :: from_path (path) . ok () , limits : super :: Limits :: default () , }) } }
};
}
