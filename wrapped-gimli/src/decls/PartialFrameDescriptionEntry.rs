macro_rules! deps {
    () => {
        FrameDescriptionEntry!();
        UnwindSection!();
        BaseAddresses!();
        Section!();
        Format!();
        Reader!();
    };
}

macro_rules! PartialFrameDescriptionEntry {
    () => {
        deps!();
        # [doc = " A partially parsed `FrameDescriptionEntry`."] # [doc = ""] # [doc = " Fully parsing this FDE requires first parsing its CIE."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct PartialFrameDescriptionEntry < 'bases , Section , R > where R : Reader , Section : UnwindSection < R > , { offset : R :: Offset , length : R :: Offset , format : Format , cie_offset : Section :: Offset , rest : R , section : Section , bases : & 'bases BaseAddresses , }
    };
}

PartialFrameDescriptionEntry!();