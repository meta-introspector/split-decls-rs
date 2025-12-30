// Generated macro for impl_55 (impl)
macro_rules! Depcrate_de_mapimpl_55 {
() => {
// Module: crate::de::map
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'de , 'd , 'm , R , E > SeqAccess < 'de > for MapValueSeqAccess < 'de , 'd , 'm , R , E > where R : XmlRead < 'de > , E : EntityResolver , { type Error = DeError ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , DeError > where T : DeserializeSeed < 'de > , { loop { self . map . skip_whitespaces () ? ; break match self . map . de . peek () ? { # [cfg (feature = "overlapped-lists")] DeEvent :: Start (e) if ! self . filter . is_suitable (e) ? => { self . map . de . skip () ? ; continue ; } # [cfg (feature = "overlapped-lists")] DeEvent :: Text (_) if self . filter . need_skip_text () => { self . map . de . skip () ? ; continue ; } # [cfg (not (feature = "overlapped-lists"))] DeEvent :: Start (e) if ! self . filter . is_suitable (e) ? => Ok (None) , # [cfg (not (feature = "overlapped-lists"))] DeEvent :: Text (_) if self . filter . need_skip_text () => Ok (None) , DeEvent :: End (e) => { debug_assert_eq ! (self . map . start . name () , e . name ()) ; Ok (None) } DeEvent :: Eof => { Err (Error :: missed_end (self . map . start . name () , self . map . start . decoder ()) . into ()) } DeEvent :: Text (_) => match self . map . de . next () ? { DeEvent :: Text (e) => seed . deserialize (TextDeserializer (e)) . map (Some) , _ => unreachable ! () , } , DeEvent :: Start (_) => match self . map . de . next () ? { DeEvent :: Start (start) => seed . deserialize (ElementDeserializer { start , de : self . map . de , }) . map (Some) , _ => unreachable ! () , } , } ; } } }
};
}
