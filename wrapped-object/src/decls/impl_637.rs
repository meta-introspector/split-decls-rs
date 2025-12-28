macro_rules! deps {
    () => {
        PeComdat!();
        ImageNtHeaders!();
        ReadRef!();
        Item!();
        PeComdatIterator!();
    };
}

macro_rules! impl_637 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > Iterator for PeComdatIterator < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type Item = PeComdat < 'data , 'file , Pe , R > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_637!()