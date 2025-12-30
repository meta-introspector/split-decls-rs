// Generated macro for LookupRefDeltaObjectsIter (struct)
macro_rules! Depcrate_data_input_lookup_ref_delta_objectsLookupRefDeltaObjectsIter {
() => {
// Module: crate::data::input::lookup_ref_delta_objects
// Provides: {"LookupRefDeltaObjectsIter"}
// Dependencies: {}
# [doc = " An iterator to resolve thin packs on the fly."] pub struct LookupRefDeltaObjectsIter < I , Find > { # [doc = " The inner iterator whose entries we will resolve."] pub inner : I , lookup : Find , # [doc = " The cached delta to provide next time we are called, it's the delta to go with the base we just resolved in its place."] next_delta : Option < input :: Entry > , # [doc = " Fuse to stop iteration after first missing object."] error : bool , # [doc = " The overall pack-offset we accumulated thus far. Each inserted entry offsets all following"] # [doc = " objects by its length. We need to determine exactly where the object was inserted to see if its affected at all."] inserted_entry_length_at_offset : Vec < Change > , # [doc = " The sum of all entries added so far, as a cache to avoid recomputation"] inserted_entries_length_in_bytes : i64 , buf : Vec < u8 > , }
};
}
