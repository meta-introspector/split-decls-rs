// Generated macro for impl_174 (impl)
macro_rules! Depcrate_data_entryimpl_174 {
() => {
// Module: crate::data::entry
// Provides: {"impl_174"}
// Dependencies: {}
# [doc = " Access"] impl Entry { # [doc = " Compute the pack offset to the base entry of the object represented by this entry."] pub fn base_pack_offset (& self , distance : u64) -> data :: Offset { let pack_offset = self . data_offset - self . header_size () as u64 ; pack_offset . checked_sub (distance) . expect ("in-bound distance of deltas") } # [doc = " The pack offset at which this entry starts"] pub fn pack_offset (& self) -> data :: Offset { self . data_offset - self . header_size () as u64 } # [doc = " The amount of bytes used to describe this entry in the pack. The header starts at [`Self::pack_offset()`]"] pub fn header_size (& self) -> usize { self . header . size (self . decompressed_size) } }
};
}
