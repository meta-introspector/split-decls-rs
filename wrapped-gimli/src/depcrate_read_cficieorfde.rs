// Generated macro for CieOrFde (enum)
macro_rules! Depcrate_read_cfiCieOrFde {
() => {
// Module: crate::read::cfi
// Provides: {"CieOrFde"}
// Dependencies: {}
# [doc = " Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE)."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum CieOrFde < 'bases , Section , R > where R : Reader , Section : UnwindSection < R > , { # [doc = " This CFI entry is a `CommonInformationEntry`."] Cie (CommonInformationEntry < R >) , # [doc = " This CFI entry is a `FrameDescriptionEntry`, however fully parsing it"] # [doc = " requires parsing its CIE first, so it is left in a partially parsed"] # [doc = " state."] Fde (PartialFrameDescriptionEntry < 'bases , Section , R >) , }
};
}
