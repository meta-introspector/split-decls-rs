macro_rules! deps {
    () => {
        Storage!();
        QueueInner!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < T , S > fmt :: Debug for QueueInner < T , S > where T : fmt :: Debug , S : Storage , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
    };
}

impl_470!();