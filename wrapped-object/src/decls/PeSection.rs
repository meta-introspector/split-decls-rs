macro_rules! deps {
    () => {
        SectionIndex!();
        PeFile!();
        ImageNtHeaders!();
        ReadRef!();
        ImageSectionHeader!();
        ObjectSection!();
    };
}

macro_rules! PeSection {
    () => {
        deps!();
        # [doc = " A section in a [`PeFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct PeSection < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { pub (super) file : & 'file PeFile < 'data , Pe , R > , pub (super) index : SectionIndex , pub (super) section : & 'data pe :: ImageSectionHeader , }
    };
}

PeSection!()