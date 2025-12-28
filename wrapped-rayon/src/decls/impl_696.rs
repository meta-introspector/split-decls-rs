macro_rules! deps {
    () => {
        MapWithIter!();
    };
}

macro_rules! impl_696 {
    () => {
        deps!();
        impl < 'f , I , U , F , R > Iterator for MapWithIter < 'f , I , U , F > where I : Iterator , F : Fn (& mut U , I :: Item) -> R + Sync , R : Send , { type Item = R ; fn next (& mut self) -> Option < R > { let item = self . base . next () ? ; Some ((self . map_op) (& mut self . item , item)) } fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } }
    };
}

impl_696!()