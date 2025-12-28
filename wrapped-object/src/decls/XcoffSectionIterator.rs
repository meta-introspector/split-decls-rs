macro_rules! deps {
    () => {
        ReadRef!();
        FileHeader!();
        XcoffFile!();
        SectionHeader!();
    };
}

macro_rules! XcoffSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in an [`XcoffFile`]."] # [derive (Debug)] pub struct XcoffSectionIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) iter : iter :: Enumerate < slice :: Iter < 'data , Xcoff :: SectionHeader > > , }
    };
}

XcoffSectionIterator!()