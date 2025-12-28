macro_rules! deps {
    () => {
        RawRwLock!();
        RwLock!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , R , T > Deserialize < 'de > for RwLock < R , T > where R : RawRwLock , T : Deserialize < 'de > + ? Sized , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (RwLock :: new) } }
    };
}

impl_107!();