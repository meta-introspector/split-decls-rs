macro_rules! deps {
    () => {
        SnapshotMut!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl Debug for SnapshotMut < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_str (& self . config . to_string ()) } }
    };
}

impl_557!();