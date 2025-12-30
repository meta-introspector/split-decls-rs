// Generated macro for impl_101 (impl)
macro_rules! Depcrate_deimpl_101 {
() => {
// Module: crate::de
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'de , 'a > de :: VariantAccess < 'de > for Enum < 'a , 'de > { type Error = Error ; fn unit_variant (self) -> Result < () > { Ok (()) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value > where T : DeserializeSeed < 'de > , { let newtype_variant = self . de . last_identifier ; self . de . parser . skip_ws () ? ; if self . de . parser . consume_char ('(') { self . de . parser . skip_ws () ? ; self . de . newtype_variant = self . de . parser . exts . contains (Extensions :: UNWRAP_VARIANT_NEWTYPES) ; let val = guard_recursion ! { self . de => seed . deserialize (& mut * self . de) . map_err (| err | struct_error_name (err , newtype_variant)) ? } ; self . de . newtype_variant = false ; self . de . parser . comma () ? ; if self . de . parser . consume_char (')') { Ok (val) } else { Err (Error :: ExpectedStructLikeEnd) } } else { Err (Error :: ExpectedStructLike) } } fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value > where V : Visitor < 'de > , { self . de . parser . skip_ws () ? ; self . de . deserialize_tuple (len , visitor) } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , visitor : V) -> Result < V :: Value > where V : Visitor < 'de > , { let struct_variant = self . de . last_identifier ; self . de . parser . skip_ws () ? ; self . de . handle_struct_after_name ("" , visitor) . map_err (| err | struct_error_name (err , struct_variant)) } }
};
}
