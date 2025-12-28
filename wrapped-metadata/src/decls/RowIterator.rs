macro_rules! deps {
    () => {
        TypeIndex!();
        AsRow!();
    };
}

macro_rules! RowIterator {
    () => {
        deps!();
        pub struct RowIterator < 'a , R : AsRow < 'a > > { index : & 'a TypeIndex , file : usize , rows : std :: ops :: Range < usize > , phantom : std :: marker :: PhantomData < R > , }
    };
}

RowIterator!();