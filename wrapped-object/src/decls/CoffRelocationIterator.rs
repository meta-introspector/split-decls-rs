macro_rules! deps {
    () => {
        ImageFileHeader!();
        CoffFile!();
        ReadRef!();
        CoffHeader!();
        ImageRelocation!();
    };
}

macro_rules! CoffRelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator for the relocations in a [`CoffSection`](super::CoffSection)."] pub struct CoffRelocationIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { pub (super) file : & 'file CoffFile < 'data , R , Coff > , pub (super) iter : slice :: Iter < 'data , pe :: ImageRelocation > , }
    };
}

CoffRelocationIterator!()