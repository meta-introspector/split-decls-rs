macro_rules! deps {
    () => {
        Rel!();
        Rela!();
        FileHeader!();
        Result!();
        ElfRelocationIterator!();
        Crel!();
        Item!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for ElfRelocationIterator < 'data , Elf > { type Item = Crel ; fn next (& mut self) -> Option < Self :: Item > { match self { ElfRelocationIterator :: Rel (ref mut i , endian) => { i . next () . map (| r | Crel :: from_rel (r , * endian)) } ElfRelocationIterator :: Rela (ref mut i , endian , is_mips64el) => { i . next () . map (| r | Crel :: from_rela (r , * endian , * is_mips64el)) } ElfRelocationIterator :: Crel (ref mut i) => i . next () . and_then (Result :: ok) , } } }
    };
}

impl_349!();