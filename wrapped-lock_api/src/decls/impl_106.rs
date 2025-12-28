macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < R , T > Serialize for RwLock < R , T > where R : RawRwLock , T : Serialize + ? Sized , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . read () . serialize (serializer) } }
    };
}

impl_106!();