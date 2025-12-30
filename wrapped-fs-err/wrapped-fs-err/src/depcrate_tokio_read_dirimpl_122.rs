// Generated macro for impl_122 (impl)
macro_rules! Depcrate_tokio_read_dirimpl_122 {
() => {
// Module: crate::tokio::read_dir
// Provides: {"impl_122"}
// Dependencies: {}
impl ReadDir { # [doc = " Returns the next entry in the directory stream."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::ReadDir::next_entry`]."] pub async fn next_entry (& mut self) -> io :: Result < Option < DirEntry > > { match self . tokio . next_entry () . await { Ok (entry) => Ok (entry . map (| e | DirEntry { tokio : e })) , Err (err) => Err (Error :: build (err , ErrorKind :: ReadDir , & self . path)) , } } # [doc = " Polls for the next directory entry in the stream."] # [doc = ""] # [doc = " Wrapper around [`tokio::fs::ReadDir::poll_next_entry`]."] pub fn poll_next_entry (& mut self , cx : & mut Context < '_ >) -> Poll < io :: Result < Option < DirEntry > > > { Poll :: Ready (match ready ! (self . tokio . poll_next_entry (cx)) { Ok (entry) => Ok (entry . map (| e | DirEntry { tokio : e })) , Err (err) => Err (Error :: build (err , ErrorKind :: ReadDir , & self . path)) , }) } }
};
}
