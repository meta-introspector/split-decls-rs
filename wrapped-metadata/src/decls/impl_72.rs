macro_rules! deps {
    () => {
        TypeIndex!();
        AsRow!();
        RowIterator!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'a , R : AsRow < 'a > > RowIterator < 'a , R > { pub (crate) fn new (index : & 'a TypeIndex , file : usize , rows : std :: ops :: Range < usize >) -> Self { Self { index , file , rows , phantom : std :: marker :: PhantomData , } } }
    };
}

impl_72!()