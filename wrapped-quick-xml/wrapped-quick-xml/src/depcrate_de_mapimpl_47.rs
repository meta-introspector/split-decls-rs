// Generated macro for impl_47 (impl)
macro_rules! Depcrate_de_mapimpl_47 {
() => {
// Module: crate::de::map
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'de , 'd , 'm , R , E > de :: EnumAccess < 'de > for MapValueDeserializer < 'de , 'd , 'm , R , E > where R : XmlRead < 'de > , E : EntityResolver , { type Error = DeError ; type Variant = MapValueVariantAccess < 'de , 'd , 'm , R , E > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let (name , is_text) = match self . map . de . peek () ? { DeEvent :: Start (e) => (seed . deserialize (QNameDeserializer :: from_elem (e) ?) ? , false) , DeEvent :: Text (_) => (seed . deserialize (BorrowedStrDeserializer :: < DeError > :: new (TEXT_KEY)) ? , true ,) , _ => unreachable ! () , } ; Ok ((name , MapValueVariantAccess { map : self . map , is_text , } ,)) } }
};
}
