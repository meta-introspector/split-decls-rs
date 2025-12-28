macro_rules! deps {
    () => {
        Section!();
        UnwindSection!();
        CommonInformationEntry!();
        PartialFrameDescriptionEntry!();
        FrameDescriptionEntry!();
        Reader!();
    };
}

macro_rules! CieOrFde {
    () => {
        deps!();
        # [doc = " Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE)."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum CieOrFde < 'bases , Section , R > where R : Reader , Section : UnwindSection < R > , { # [doc = " This CFI entry is a `CommonInformationEntry`."] Cie (CommonInformationEntry < R >) , # [doc = " This CFI entry is a `FrameDescriptionEntry`, however fully parsing it"] # [doc = " requires parsing its CIE first, so it is left in a partially parsed"] # [doc = " state."] Fde (PartialFrameDescriptionEntry < 'bases , Section , R >) , }
    };
}

CieOrFde!();