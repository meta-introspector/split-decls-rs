macro_rules! deps {
    () => {
        ReadRef!();
        CoffHeader!();
        ImageSectionHeader!();
        CoffFile!();
        ImageFileHeader!();
    };
}

macro_rules! CoffSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`CoffFile`]."] # [derive (Debug)] pub struct CoffSectionIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { pub (super) file : & 'file CoffFile < 'data , R , Coff > , pub (super) iter : iter :: Enumerate < slice :: Iter < 'data , pe :: ImageSectionHeader > > , }
    };
}

CoffSectionIterator!();