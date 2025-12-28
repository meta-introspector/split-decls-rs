macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < 'de , K , V , S > Deserialize < 'de > for LinkedHashMap < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : BuildHasher + Default , { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { # [derive (Debug)] pub struct LinkedHashMapVisitor < K , V , S > { marker : PhantomData < LinkedHashMap < K , V , S > > , } impl < K , V , S > LinkedHashMapVisitor < K , V , S > { fn new () -> Self { LinkedHashMapVisitor { marker : PhantomData , } } } impl < K , V , S > Default for LinkedHashMapVisitor < K , V , S > { fn default () -> Self { Self :: new () } } impl < 'de , K , V , S > Visitor < 'de > for LinkedHashMapVisitor < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : BuildHasher + Default , { type Value = LinkedHashMap < K , V , S > ; fn expecting (& self , formatter : & mut Formatter) -> fmt :: Result { write ! (formatter , "a map") } # [inline] fn visit_map < M : MapAccess < 'de > > (self , mut map : M) -> Result < Self :: Value , M :: Error > { let mut values = LinkedHashMap :: with_capacity_and_hasher (map . size_hint () . unwrap_or (0) , S :: default () ,) ; while let Some ((k , v)) = map . next_entry () ? { values . insert (k , v) ; } Ok (values) } } deserializer . deserialize_map (LinkedHashMapVisitor :: default ()) } }
    };
}

impl_194!()