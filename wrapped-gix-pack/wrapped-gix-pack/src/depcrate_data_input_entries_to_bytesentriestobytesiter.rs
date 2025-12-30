// Generated macro for EntriesToBytesIter (struct)
macro_rules! Depcrate_data_input_entries_to_bytesEntriesToBytesIter {
() => {
// Module: crate::data::input::entries_to_bytes
// Provides: {"EntriesToBytesIter"}
// Dependencies: {}
# [doc = " An implementation of [`Iterator`] to write [encoded entries][input::Entry] to an inner implementation each time"] # [doc = " `next()` is called."] # [doc = ""] # [doc = " It is able to deal with an unknown amount of objects as it will rewrite the pack header once the entries iterator"] # [doc = " is depleted and compute the hash in one go by re-reading the whole file."] pub struct EntriesToBytesIter < I : Iterator , W > { # [doc = " An iterator for input [`input::Entry`] instances"] pub input : Peekable < I > , # [doc = " A way of writing encoded bytes."] output : W , # [doc = " Our trailing hash when done writing all input entries"] trailer : Option < gix_hash :: ObjectId > , # [doc = " The amount of objects in the iteration and the version of the packfile to be written."] # [doc = " Will be `None` to signal the header was written already."] data_version : crate :: data :: Version , # [doc = " The amount of entries seen so far"] num_entries : u32 , # [doc = " If we are done, no additional writes will occur"] is_done : bool , # [doc = " The kind of hash to use for the digest"] object_hash : gix_hash :: Kind , }
};
}
