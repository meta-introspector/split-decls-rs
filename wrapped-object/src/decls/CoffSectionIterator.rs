macro_rules! deps {
    () => {
        ReadRef!();
        ImageFileHeader!();
        CoffFile!();
        ImageSectionHeader!();
        CoffHeader!();
    };
}

macro_rules! CoffSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`CoffFile`]."] # [derive (Debug)] pub struct CoffSectionIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { pub (super) file : & 'file CoffFile < 'data , R , Coff > , pub (super) iter : iter :: Enumerate < slice :: Iter < 'data , pe :: ImageSectionHeader > > , }
    };
}

CoffSectionIterator!()