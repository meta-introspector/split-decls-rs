macro_rules! deps {
    () => {
        Generation!();
        Store!();
    };
}

macro_rules! SlotMapIndex {
    () => {
        deps!();
        # [doc = " An index that changes only if the packs directory changes and its contents is re-read."] # [derive (Default)] pub struct SlotMapIndex { # [doc = " The index into the slot map at which we expect an index or pack file. Neither of these might be already loaded."] pub (crate) slot_indices : Vec < usize > , # [doc = " A list of loose object databases as resolved by their alternates file in the `object_directory`."] # [doc = " The first entry is this repository's directory for the loose file database."] # [doc = " All other entries are the loose stores of alternates."] # [doc = " It's in an Arc to be shared to Handles, but not to be shared across SlotMapIndices."] pub (crate) loose_dbs : Arc < Vec < crate :: loose :: Store > > , # [doc = " A static value that doesn't ever change for a particular clone of this index."] pub (crate) generation : Generation , # [doc = " The number of indices loaded thus far when the index of the slot map was last examined, which can change as new indices are loaded"] # [doc = " in parallel."] # [doc = " Shared across SlotMapIndex instances of the same generation."] pub (crate) next_index_to_load : Arc < AtomicUsize > , # [doc = " Incremented by one up to `slot_indices.len()` once an attempt to load an index completed."] # [doc = " If a load failed, there will also be an increment."] # [doc = " Shared across SlotMapIndex instances of the same generation."] pub (crate) loaded_indices : Arc < AtomicUsize > , # [doc = " The amount of indices that are currently being loaded."] # [doc = " Zero if no loading operation is currently happening, or more otherwise."] pub (crate) num_indices_currently_being_loaded : Arc < AtomicU16 > , }
    };
}

SlotMapIndex!()