macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'de , T , S , E > IntoDeserializer < 'de , E > for IndexSet < T , S > where T : IntoDeserializer < 'de , E > + Eq + Hash , S : BuildHasher , E : Error , { type Deserializer = SeqDeserializer < < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { SeqDeserializer :: new (self . into_iter ()) } }
    };
}

impl_31!()