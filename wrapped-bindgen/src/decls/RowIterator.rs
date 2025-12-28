macro_rules! deps {
    () => {
        File!();
        AsRow!();
    };
}

macro_rules! RowIterator {
    () => {
        deps!();
        pub struct RowIterator < R : AsRow > { file : & 'static File , rows : std :: ops :: Range < usize > , phantom : std :: marker :: PhantomData < R > , }
    };
}

RowIterator!();