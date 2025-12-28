macro_rules! deps {
    () => {
        Error!();
        Find!();
        Kind!();
    };
}

macro_rules! existing_iter {
    () => {
        deps!();
        # [doc = ""] pub mod existing_iter { use gix_hash :: ObjectId ; # [doc = " The error returned by the various [`find_*_iter()`][crate::FindExt::find_commit_iter()] trait methods."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Find (crate :: find :: Error) , # [error ("An object with id {oid} could not be found")] NotFound { oid : ObjectId } , # [error ("Expected object of kind {expected} but got {actual} at {oid}")] ObjectKind { oid : ObjectId , actual : crate :: Kind , expected : crate :: Kind , } , } }
    };
}

existing_iter!();