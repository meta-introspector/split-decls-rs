macro_rules! deps {
    () => {
        Storage!();
        QueueInner!();
    };
}

macro_rules! impl_471 {
    () => {
        deps!();
        impl < T , S > hash :: Hash for QueueInner < T , S > where T : hash :: Hash , S : Storage , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { for t in self . iter () { hash :: Hash :: hash (t , state) ; } } }
    };
}

impl_471!()