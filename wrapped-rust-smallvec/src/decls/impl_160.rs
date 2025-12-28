macro_rules! deps {
    () => {
        SmallVec!();
        SmallVecVisitor!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] impl < 'de , T , const N : usize > Deserialize < 'de > for SmallVec < T , N > where T : Deserialize < 'de > , { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { deserializer . deserialize_seq (SmallVecVisitor { phantom : PhantomData , }) } }
    };
}

impl_160!()