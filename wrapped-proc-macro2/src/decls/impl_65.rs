macro_rules! deps {
    () => {
        RcVecMut!();
        RcVecBuilder!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < 'a , T > RcVecMut < 'a , T > { pub (crate) fn push (& mut self , element : T) { self . inner . push (element) ; } pub (crate) fn extend (& mut self , iter : impl IntoIterator < Item = T >) { self . inner . extend (iter) ; } pub (crate) fn as_mut (& mut self) -> RcVecMut < T > { RcVecMut { inner : self . inner } } pub (crate) fn take (self) -> RcVecBuilder < T > { let vec = mem :: take (self . inner) ; RcVecBuilder { inner : vec } } }
    };
}

impl_65!()