macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! integrity {
    () => {
        deps!();
        # [doc = ""] pub mod integrity { # [doc = " The error returned by [`verify_integrity()`][super::Store::verify_integrity()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("{kind} object {id} could not be decoded")] ObjectDecode { source : gix_object :: decode :: Error , kind : gix_object :: Kind , id : gix_hash :: ObjectId , } , # [error ("{kind} object {expected} could not be hashed")] ObjectHasher { # [source] source : gix_hash :: hasher :: Error , kind : gix_object :: Kind , expected : gix_hash :: ObjectId , } , # [error ("{kind} object wasn't re-encoded without change")] ObjectEncodeMismatch { # [source] source : gix_hash :: verify :: Error , kind : gix_object :: Kind , } , # [error ("Objects were deleted during iteration - try again")] Retry , # [error ("Interrupted")] Interrupted , } # [doc = " The outcome returned by [`verify_integrity()`][super::Store::verify_integrity()]."] # [derive (Debug , PartialEq , Eq , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Statistics { # [doc = " The amount of loose objects we checked."] pub num_objects : usize , } # [doc = " The progress ids used in [`verify_integrity()`][super::Store::verify_integrity()]."] # [doc = ""] # [doc = " Use this information to selectively extract the progress of interest in case the parent application has custom visualization."] # [derive (Debug , Copy , Clone)] pub enum ProgressId { # [doc = " The amount of loose objects that have been verified."] LooseObjects , } impl From < ProgressId > for gix_features :: progress :: Id { fn from (v : ProgressId) -> Self { match v { ProgressId :: LooseObjects => * b"VILO" , } } } }
    };
}

integrity!();