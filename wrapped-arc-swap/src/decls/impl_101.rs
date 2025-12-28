macro_rules! deps {
    () => {
        ArcSwapAny!();
        Strategy!();
        RefCnt!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'de , T , S > Deserialize < 'de > for ArcSwapAny < T , S > where T : RefCnt + Deserialize < 'de > , S : Strategy < T > + Default , { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (Self :: from (T :: deserialize (deserializer) ?)) } }
    };
}

impl_101!()