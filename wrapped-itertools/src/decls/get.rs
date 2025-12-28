macro_rules! deps {
    () => {
        IteratorIndex!();
    };
}

macro_rules! get {
    () => {
        deps!();
        pub fn get < I , R > (iter : I , index : R) -> R :: Output where I : IntoIterator , R : IteratorIndex < I :: IntoIter > , { index . index (iter . into_iter ()) }
    };
}

get!();