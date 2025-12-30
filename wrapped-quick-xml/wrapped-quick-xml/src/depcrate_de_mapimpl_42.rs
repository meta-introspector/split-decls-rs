// Generated macro for impl_42 (impl)
macro_rules! Depcrate_de_mapimpl_42 {
() => {
// Module: crate::de::map
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'de , 'd , R , E > ElementMapAccess < 'de , 'd , R , E > where R : XmlRead < 'de > , E : EntityResolver , { # [doc = " Create a new ElementMapAccess"] pub fn new (de : & 'd mut Deserializer < 'de , R , E > , start : BytesStart < 'de > , fields : & 'static [& 'static str] ,) -> Self { Self { de , iter : IterState :: new (start . name () . as_ref () . len () , false) , start , source : ValueSource :: Unknown , fields , has_value_field : fields . contains (& VALUE_KEY) , has_text_field : fields . contains (& TEXT_KEY) , } } # [doc = " Determines if subtree started with the specified event shoould be skipped."] # [doc = ""] # [doc = " Used to map elements with `xsi:nil` attribute set to true to `None` in optional contexts."] # [doc = ""] # [doc = " We need to handle two attributes:"] # [doc = " - on parent element: `<map xsi:nil=\"true\"><foo/></map>`"] # [doc = " - on this element:   `<map><foo xsi:nil=\"true\"/></map>`"] # [doc = ""] # [doc = " We check parent element too because `xsi:nil` affects only nested elements of the"] # [doc = " tag where it is defined. We can map structure with fields mapped to attributes to"] # [doc = " the `<map>` element and set to `None` all its optional elements."] fn should_skip_subtree (& self , start : & BytesStart) -> bool { self . de . reader . reader . has_nil_attr (& self . start) || self . de . reader . reader . has_nil_attr (start) } # [doc = " Skips whitespaces when they are not preserved"] # [inline] fn skip_whitespaces (& mut self) -> Result < () , DeError > { self . de . skip_whitespaces () } }
};
}
