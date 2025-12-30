// Generated macro for impl_153 (impl)
macro_rules! Depcrate_deimpl_153 {
() => {
// Module: crate::de
// Provides: {"impl_153"}
// Dependencies: {}
# [doc = " An accessor to sequence elements forming a value for top-level sequence of XML"] # [doc = " elements."] # [doc = ""] # [doc = " Technically, multiple top-level elements violates XML rule of only one top-level"] # [doc = " element, but we consider this as several concatenated XML documents."] impl < 'de , R , E > SeqAccess < 'de > for & mut Deserializer < 'de , R , E > where R : XmlRead < 'de > , E : EntityResolver , { type Error = DeError ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > , { self . skip_whitespaces () ? ; match self . peek () ? { DeEvent :: Eof => Ok (None) , _ => seed . deserialize (& mut * * self) . map (Some) , } } }
};
}
