macro_rules! deps {
    () => {
        ObjectSegment!();
        ImageSectionHeader!();
        ImageNtHeaders!();
        ReadRef!();
        PeFile!();
    };
}

macro_rules! PeSegment {
    () => {
        deps!();
        # [doc = " A loadable section in a [`PeFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] # [derive (Debug)] pub struct PeSegment < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { file : & 'file PeFile < 'data , Pe , R > , section : & 'data pe :: ImageSectionHeader , }
    };
}

PeSegment!();