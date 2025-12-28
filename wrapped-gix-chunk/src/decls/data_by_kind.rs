macro_rules! data_by_kind {
    () => {
        # [doc = ""] pub mod data_by_kind { # [doc = " The error returned by [`Index::data_by_id()`][super::Index::data_by_id()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The chunk wasn't found in the file index")] NotFound (# [from] super :: offset_by_kind :: Error) , # [error ("The offsets into the file couldn't be represented by usize")] FileTooLarge , } }
    };
}

data_by_kind!();