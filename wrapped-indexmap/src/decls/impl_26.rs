macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'de , K , V , S , E > IntoDeserializer < 'de , E > for IndexMap < K , V , S > where K : IntoDeserializer < 'de , E > + Eq + Hash , V : IntoDeserializer < 'de , E > , S : BuildHasher , E : Error , { type Deserializer = MapDeserializer < 'de , < Self as IntoIterator > :: IntoIter , E > ; fn into_deserializer (self) -> Self :: Deserializer { MapDeserializer :: new (self . into_iter ()) } }
    };
}

impl_26!();