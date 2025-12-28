macro_rules! deps {
    () => {
        Item!();
        ReadRef!();
        DyldCacheImageIterator!();
        Endian!();
        DyldCacheImage!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl < 'data , 'cache , E , R > Iterator for DyldCacheImageIterator < 'data , 'cache , E , R > where E : Endian , R : ReadRef < 'data > , { type Item = DyldCacheImage < 'data , 'cache , E , R > ; fn next (& mut self) -> Option < DyldCacheImage < 'data , 'cache , E , R > > { let image_info = self . iter . next () ? ; Some (DyldCacheImage { cache : self . cache , image_info , }) } }
    };
}

impl_474!();