// Generated macro for impl_212 (impl)
macro_rules! Depcrate_serdeimpl_212 {
() => {
// Module: crate::serde
// Provides: {"impl_212"}
// Dependencies: {}
impl < 'de , T , S > Deserialize < 'de > for LinkedHashSet < T , S > where T : Deserialize < 'de > + Eq + Hash , S : BuildHasher + Default , { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { # [derive (Debug)] pub struct LinkedHashSetVisitor < T , S > { marker : PhantomData < LinkedHashSet < T , S > > , } impl < T , S > LinkedHashSetVisitor < T , S > { fn new () -> Self { LinkedHashSetVisitor { marker : PhantomData , } } } impl < T , S > Default for LinkedHashSetVisitor < T , S > { fn default () -> Self { Self :: new () } } impl < 'de , T , S > Visitor < 'de > for LinkedHashSetVisitor < T , S > where T : Deserialize < 'de > + Eq + Hash , S : BuildHasher + Default , { type Value = LinkedHashSet < T , S > ; fn expecting (& self , formatter : & mut Formatter) -> fmt :: Result { write ! (formatter , "a sequence") } # [inline] fn visit_seq < SA : SeqAccess < 'de > > (self , mut seq : SA) -> Result < Self :: Value , SA :: Error > { let mut values = LinkedHashSet :: with_capacity_and_hasher (seq . size_hint () . unwrap_or (0) , S :: default () ,) ; while let Some (v) = seq . next_element () ? { values . insert (v) ; } Ok (values) } } deserializer . deserialize_seq (LinkedHashSetVisitor :: default ()) } }
};
}
