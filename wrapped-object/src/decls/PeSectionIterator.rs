macro_rules! deps {
    () => {
        ImageSectionHeader!();
        PeFile!();
        ImageNtHeaders!();
        ReadRef!();
    };
}

macro_rules! PeSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`PeFile`]."] # [derive (Debug)] pub struct PeSectionIterator < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { pub (super) file : & 'file PeFile < 'data , Pe , R > , pub (super) iter : iter :: Enumerate < slice :: Iter < 'data , pe :: ImageSectionHeader > > , }
    };
}

PeSectionIterator!();