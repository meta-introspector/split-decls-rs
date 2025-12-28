macro_rules! deps {
    () => {
        Rewind!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T > Rewind < T > { # [cfg (test)] pub (crate) fn new (io : T) -> Self { Rewind { pre : None , inner : io , } } pub (crate) fn new_buffered (io : T , buf : Bytes) -> Self { Rewind { pre : Some (buf) , inner : io , } } # [cfg (test)] pub (crate) fn rewind (& mut self , bs : Bytes) { debug_assert ! (self . pre . is_none ()) ; self . pre = Some (bs) ; } pub (crate) fn into_inner (self) -> (T , Bytes) { (self . inner , self . pre . unwrap_or_default ()) } }
    };
}

impl_74!();