// Generated macro for offset_by_kind (module)
macro_rules! Depcrate_file_indexoffset_by_kind {
() => {
// Module: crate::file::index
// Provides: {"offset_by_kind"}
// Dependencies: {}
# [doc = ""] pub mod offset_by_kind { use std :: fmt :: { Display , Formatter } ; # [doc = " The error returned by [`Index::offset_by_id()`][super::Index::offset_by_id()]."] # [allow (missing_docs)] # [derive (Debug)] pub struct Error { pub kind : crate :: Id , } impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Chunk named {:?} was not found in chunk file index" , std :: str :: from_utf8 (& self . kind) . unwrap_or ("<non-ascii>")) } } impl std :: error :: Error for Error { } }
};
}
