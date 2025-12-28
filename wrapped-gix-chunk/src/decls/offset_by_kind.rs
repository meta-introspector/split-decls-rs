macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! offset_by_kind {
    () => {
        deps!();
        # [doc = ""] pub mod offset_by_kind { use std :: fmt :: { Display , Formatter } ; # [doc = " The error returned by [`Index::offset_by_id()`][super::Index::offset_by_id()]."] # [allow (missing_docs)] # [derive (Debug)] pub struct Error { pub kind : crate :: Id , } impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Chunk named {:?} was not found in chunk file index" , std :: str :: from_utf8 (& self . kind) . unwrap_or ("<non-ascii>")) } } impl std :: error :: Error for Error { } }
    };
}

offset_by_kind!()