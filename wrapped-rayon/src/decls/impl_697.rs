macro_rules! deps {
    () => {
        MapWithIter!();
    };
}

macro_rules! impl_697 {
    () => {
        deps!();
        impl < 'f , I , U , F , R > DoubleEndedIterator for MapWithIter < 'f , I , U , F > where I : DoubleEndedIterator , F : Fn (& mut U , I :: Item) -> R + Sync , R : Send , { fn next_back (& mut self) -> Option < R > { let item = self . base . next_back () ? ; Some ((self . map_op) (& mut self . item , item)) } }
    };
}

impl_697!()