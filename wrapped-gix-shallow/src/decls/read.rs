macro_rules! read {
    () => {
        # [doc = ""] pub mod read { # [doc = " The error returned by [`read`](crate::read())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not open shallow file for reading")] Io (# [from] std :: io :: Error) , # [error ("Could not decode a line in shallow file as hex-encoded object hash")] DecodeHash (# [from] gix_hash :: decode :: Error) , } }
    };
}

read!()