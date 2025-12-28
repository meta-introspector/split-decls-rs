macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , T , A > serde :: Deserialize < 'de > for Box < T , A > where T : serde :: Deserialize < 'de > , A : Allocator + Default , { # [inline (always)] fn deserialize < D : serde :: de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { let value = T :: deserialize (deserializer) ? ; Ok (Box :: new_in (value , A :: default ())) } }
    };
}

impl_79!()