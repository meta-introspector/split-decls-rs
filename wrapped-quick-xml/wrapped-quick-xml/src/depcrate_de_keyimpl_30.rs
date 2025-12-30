// Generated macro for impl_30 (impl)
macro_rules! Depcrate_de_keyimpl_30 {
() => {
// Module: crate::de::key
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'i , 'd > QNameDeserializer < 'i , 'd > { # [doc = " Creates deserializer from name of an attribute"] pub fn from_attr (name : QName < 'd > , decoder : Decoder , key_buf : & 'd mut String ,) -> Result < Self , DeError > { if name . as_namespace_binding () . is_some () { decoder . decode_into (name . into_inner () , key_buf) ? ; } else { let (local , prefix_opt) = name . decompose () ; if prefix_opt . map_or (false , | prefix | prefix . is_xml ()) { decoder . decode_into (name . into_inner () , key_buf) ? ; } else { decoder . decode_into (local . into_inner () , key_buf) ? ; } } ; Ok (Self { name : CowRef :: Slice (key_buf) , }) } # [doc = " Creates deserializer from name of an element"] pub fn from_elem (start : & 'd BytesStart < 'i >) -> Result < Self , DeError > { let local = match start . buf { Cow :: Borrowed (b) => match decode_name (QName (& b [.. start . name_len]) , start . decoder ()) ? { Cow :: Borrowed (borrowed) => CowRef :: Input (borrowed) , Cow :: Owned (owned) => CowRef :: Owned (owned) , } , Cow :: Owned (ref o) => match decode_name (QName (& o [.. start . name_len]) , start . decoder ()) ? { Cow :: Borrowed (borrowed) => CowRef :: Slice (borrowed) , Cow :: Owned (owned) => CowRef :: Owned (owned) , } , } ; Ok (Self { name : local }) } }
};
}
