macro_rules! deps {
    () => {
        ReentrantMutex!();
        RawMutex!();
        GetThreadId!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < R , G , T > Serialize for ReentrantMutex < R , G , T > where R : RawMutex , G : GetThreadId , T : Serialize + ? Sized , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . lock () . serialize (serializer) } }
    };
}

impl_69!();