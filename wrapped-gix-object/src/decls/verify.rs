macro_rules! deps {
    () => {
        Data!();
        Error!();
    };
}

macro_rules! verify {
    () => {
        deps!();
        # [doc = " Types supporting object hash verification"] pub mod verify { # [doc = " Returned by [`crate::Data::verify_checksum()`]"] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to hash object")] Hasher (# [from] gix_hash :: hasher :: Error) , # [error (transparent)] Verify (# [from] gix_hash :: verify :: Error) , } impl crate :: Data < '_ > { # [doc = " Compute the checksum of `self` and compare it with the `expected` hash."] # [doc = " If the hashes do not match, an [`Error`] is returned, containing the actual"] # [doc = " hash of `self`."] pub fn verify_checksum (& self , expected : & gix_hash :: oid) -> Result < gix_hash :: ObjectId , Error > { let actual = crate :: compute_hash (expected . kind () , self . kind , self . data) ? ; actual . verify (expected) ? ; Ok (actual) } } }
    };
}

verify!()