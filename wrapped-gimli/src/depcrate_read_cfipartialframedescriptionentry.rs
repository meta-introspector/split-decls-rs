// Generated macro for PartialFrameDescriptionEntry (struct)
macro_rules! Depcrate_read_cfiPartialFrameDescriptionEntry {
() => {
// Module: crate::read::cfi
// Provides: {"PartialFrameDescriptionEntry"}
// Dependencies: {}
# [doc = " A partially parsed `FrameDescriptionEntry`."] # [doc = ""] # [doc = " Fully parsing this FDE requires first parsing its CIE."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct PartialFrameDescriptionEntry < 'bases , Section , R > where R : Reader , Section : UnwindSection < R > , { offset : R :: Offset , length : R :: Offset , format : Format , cie_offset : Section :: Offset , rest : R , section : Section , bases : & 'bases BaseAddresses , }
};
}
