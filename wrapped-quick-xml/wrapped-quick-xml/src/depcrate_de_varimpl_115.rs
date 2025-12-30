// Generated macro for impl_115 (impl)
macro_rules! Depcrate_de_varimpl_115 {
() => {
// Module: crate::de::var
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'de , 'd , R , E > de :: EnumAccess < 'de > for EnumAccess < 'de , 'd , R , E > where R : XmlRead < 'de > , E : EntityResolver , { type Error = DeError ; type Variant = VariantAccess < 'de , 'd , R , E > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let (name , is_text) = match self . de . peek () ? { DeEvent :: Start (e) => (seed . deserialize (QNameDeserializer :: from_elem (e) ?) ? , false) , DeEvent :: Text (_) => (seed . deserialize (BorrowedStrDeserializer :: < DeError > :: new (TEXT_KEY)) ? , true ,) , DeEvent :: End (e) => unreachable ! ("{:?}" , e) , DeEvent :: Eof => return Err (DeError :: UnexpectedEof) , } ; Ok ((name , VariantAccess { de : self . de , is_text , } ,)) } }
};
}
