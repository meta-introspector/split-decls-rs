// Generated macro for ExceptionBitsULE (struct)
macro_rules! Depcrate_provider_exception_helpersExceptionBitsULE {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"ExceptionBitsULE"}
// Dependencies: {}
# [doc = " The bitflags on an exception header."] # [doc = ""] # [doc = " Format from icu4c, documented in casepropsbuilder.cpp, shifted 8 bits since ICU4C has this packed"] # [doc = " alongside a SlotPresence"] # [doc = ""] # [doc = " ```text"] # [doc = "            0  Double-width slots. If set, then each optional slot is stored as two"] # [doc = "               elements of the array (high and low halves of 32-bit values) instead of"] # [doc = "               a single element."] # [doc = "            1  Has no simple case folding, even if there is a simple lowercase mapping"] # [doc = "           2  The value in the delta slot is negative"] # [doc = "           3  Is case-sensitive (not exposed)"] # [doc = "       4..5  Dot type"] # [doc = "           6  Has conditional special casing"] # [doc = "           7  Has conditional case folding"] # [doc = " ```"] # [doc = ""] # [doc = " All bits are valid, though in ICU4X data bits 0 and 2 are not used"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Copy , Clone , PartialEq , Eq , ULE , Debug)] # [repr (transparent)] pub struct ExceptionBitsULE (pub u8) ;
};
}
