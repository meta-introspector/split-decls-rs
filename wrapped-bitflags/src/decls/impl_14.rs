macro_rules! deps {
    () => {
        WriteHex!();
        AsDisplay!();
        Bits!();
        Flags!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'a , B : Flags > fmt :: Display for AsDisplay < 'a , B > where B :: Bits : WriteHex , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { to_writer (self . 0 , f) } }
    };
}

impl_14!();