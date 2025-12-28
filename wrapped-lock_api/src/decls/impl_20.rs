macro_rules! deps {
    () => {
        RawMutex!();
        Mutex!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , R , T > Deserialize < 'de > for Mutex < R , T > where R : RawMutex , T : Deserialize < 'de > + ? Sized , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (Mutex :: new) } }
    };
}

impl_20!();