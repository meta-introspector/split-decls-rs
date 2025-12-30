// Generated macro for SlotPresence (struct)
macro_rules! Depcrate_provider_exception_helpersSlotPresence {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"SlotPresence"}
// Dependencies: {}
# [doc = " Packed slot presence marker"] # [doc = ""] # [doc = " All bits are valid, though bit 4 is unused and reserved"] # [doc = ""] # [doc = " Bits:"] # [doc = ""] # [doc = " ```text"] # [doc = "               0: Lowercase mapping (code point)"] # [doc = "               1: Case folding (code point)"] # [doc = "               2: Uppercase mapping (code point)"] # [doc = "               3: Titlecase mapping (code point)"] # [doc = "               4: Delta to simple case mapping (code point) (sign stored separately)"] # [doc = "               5: RESERVED"] # [doc = "               6: Closure mappings (string; see below)"] # [doc = "               7: Full mappings (strings; see below)"] # [doc = " ```"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Copy , Clone , PartialEq , Eq , ULE , Debug , Default)] # [repr (transparent)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] pub struct SlotPresence (pub u8) ;
};
}
