// Generated macro for FromEntriesIter (struct)
macro_rules! Depcrate_data_output_bytesFromEntriesIter {
() => {
// Module: crate::data::output::bytes
// Provides: {"FromEntriesIter"}
// Dependencies: {}
# [doc = " An implementation of [`Iterator`] to write [encoded entries][output::Entry] to an inner implementation each time"] # [doc = " `next()` is called."] pub struct FromEntriesIter < I , W > { # [doc = " An iterator for input [`output::Entry`] instances"] pub input : I , # [doc = " A way of writing encoded bytes."] output : gix_hash :: io :: Write < W > , # [doc = " Our trailing hash when done writing all input entries"] trailer : Option < gix_hash :: ObjectId > , # [doc = " The amount of objects in the iteration and the version of the packfile to be written."] # [doc = " Will be `None` to signal the header was written already."] header_info : Option < (crate :: data :: Version , u32) > , # [doc = " The pack data version with which pack entries should be written."] entry_version : crate :: data :: Version , # [doc = " The amount of written bytes thus far"] written : u64 , # [doc = " Required to quickly find offsets by object IDs, as future objects may refer to those in the past to become a delta offset base."] # [doc = " It stores the pack offsets at which objects begin."] # [doc = " Additionally we store if an object was invalid, and if so we will not write it nor will we allow delta objects to it."] pack_offsets_and_validity : Vec < (u64 , bool) > , # [doc = " If we are done, no additional writes will occur"] is_done : bool , }
};
}
