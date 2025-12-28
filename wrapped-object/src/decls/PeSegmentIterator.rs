macro_rules! deps {
    () => {
        ImageSectionHeader!();
        PeFile!();
        ImageNtHeaders!();
        ReadRef!();
    };
}

macro_rules! PeSegmentIterator {
    () => {
        deps!();
        # [doc = " An iterator for the loadable sections in a [`PeFile`]."] # [derive (Debug)] pub struct PeSegmentIterator < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { pub (super) file : & 'file PeFile < 'data , Pe , R > , pub (super) iter : slice :: Iter < 'data , pe :: ImageSectionHeader > , }
    };
}

PeSegmentIterator!();