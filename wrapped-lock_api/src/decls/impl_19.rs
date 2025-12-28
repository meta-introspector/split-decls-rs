macro_rules! deps {
    () => {
        RawMutex!();
        Mutex!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < R , T > Serialize for Mutex < R , T > where R : RawMutex , T : Serialize + ? Sized , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . lock () . serialize (serializer) } }
    };
}

impl_19!();