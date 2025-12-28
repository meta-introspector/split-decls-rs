macro_rules! deps {
    () => {
        CoffFile!();
        ImageSectionHeader!();
        CoffHeader!();
        ImageFileHeader!();
        ReadRef!();
    };
}

macro_rules! CoffSegmentIterator {
    () => {
        deps!();
        # [doc = " An iterator for the loadable sections in a [`CoffFile`]."] # [derive (Debug)] pub struct CoffSegmentIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { pub (super) file : & 'file CoffFile < 'data , R , Coff > , pub (super) iter : slice :: Iter < 'data , pe :: ImageSectionHeader > , }
    };
}

CoffSegmentIterator!()