macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! checksum {
    () => {
        deps!();
        # [doc = ""] pub mod checksum { # [doc = " The error used in [`super::File::verify_checksum()`]."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("failed to hash commit graph file")] Hasher (# [from] gix_hash :: hasher :: Error) , # [error (transparent)] Verify (# [from] gix_hash :: verify :: Error) , } }
    };
}

checksum!();