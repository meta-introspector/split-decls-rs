// Generated macro for impl_892 (impl)
macro_rules! Depcrate_http_multipartimpl_892 {
() => {
// Module: crate::http::multipart
// Provides: {"impl_892"}
// Dependencies: {}
impl MultipartOptions { # [doc = " Set maximum file size."] # [must_use] pub fn max_file_size (self , size : usize) -> Self { MultipartOptions { max_file_size : Some (size) , .. self } } # [doc = " Set maximum number of files."] # [must_use] pub fn max_num_files (self , n : usize) -> Self { MultipartOptions { max_num_files : Some (n) , .. self } } }
};
}
