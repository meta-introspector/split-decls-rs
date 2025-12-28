macro_rules! deps {
    () => {
        RcVecMut!();
        RcVec!();
        RcVecBuilder!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T > RcVecBuilder < T > { pub (crate) fn new () -> Self { RcVecBuilder { inner : Vec :: new () } } pub (crate) fn with_capacity (cap : usize) -> Self { RcVecBuilder { inner : Vec :: with_capacity (cap) , } } pub (crate) fn push (& mut self , element : T) { self . inner . push (element) ; } pub (crate) fn extend (& mut self , iter : impl IntoIterator < Item = T >) { self . inner . extend (iter) ; } pub (crate) fn as_mut (& mut self) -> RcVecMut < T > { RcVecMut { inner : & mut self . inner , } } pub (crate) fn build (self) -> RcVec < T > { RcVec { inner : Rc :: new (self . inner) , } } }
    };
}

impl_64!()