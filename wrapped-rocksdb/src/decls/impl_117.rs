macro_rules! deps {
    () => {
        DBInner!();
        DBCommon!();
        ThreadMode!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T : ThreadMode , I : DBInner > fmt :: Debug for DBCommon < T , I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "RocksDB {{ path: {:?} }}" , self . path ()) } }
    };
}

impl_117!();