macro_rules! RcVecMut {
    () => {
        pub (crate) struct RcVecMut < 'a , T > { inner : & 'a mut Vec < T > , }
    };
}

RcVecMut!()