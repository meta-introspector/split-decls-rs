macro_rules! EditionedFileId {
    () => {
        # [salsa_macros :: interned (no_lifetime , debug , constructor = from_span , revisions = usize :: MAX)] # [derive (PartialOrd , Ord)] pub struct EditionedFileId { pub editioned_file_id : span :: EditionedFileId , }
    };
}

EditionedFileId!();