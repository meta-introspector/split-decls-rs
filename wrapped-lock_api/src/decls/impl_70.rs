macro_rules! deps {
    () => {
        RawMutex!();
        ReentrantMutex!();
        GetThreadId!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , R , G , T > Deserialize < 'de > for ReentrantMutex < R , G , T > where R : RawMutex , G : GetThreadId , T : Deserialize < 'de > + ? Sized , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (ReentrantMutex :: new) } }
    };
}

impl_70!();