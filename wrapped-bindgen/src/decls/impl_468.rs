macro_rules! deps {
    () => {
        File!();
        AsRow!();
        RowIterator!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl < R : AsRow > RowIterator < R > { pub (crate) fn new (file : & 'static File , rows : std :: ops :: Range < usize >) -> Self { Self { file , rows , phantom : std :: marker :: PhantomData , } } }
    };
}

impl_468!();