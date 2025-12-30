// Generated macro for impl_96 (impl)
macro_rules! Depcrate_deimpl_96 {
() => {
// Module: crate::de
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'de , 'a > de :: SeqAccess < 'de > for CommaSeparated < 'a , 'de > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > > where T : DeserializeSeed < 'de > , { if self . has_element () ? { let res = guard_recursion ! { self . de => seed . deserialize (& mut * self . de) ? } ; self . had_comma = self . de . parser . comma () ? ; Ok (Some (res)) } else { Ok (None) } } }
};
}
