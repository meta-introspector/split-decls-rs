// Generated macro for impl_418 (impl)
macro_rules! Depcrate_handlesimpl_418 {
() => {
// Module: crate::handles
// Provides: {"impl_418"}
// Dependencies: {}
impl < 'a , 'b > Utf8ReadHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (source : & 'a mut Utf8Source < 'b >) -> Utf8ReadHandle < 'a , 'b > { Utf8ReadHandle { source } } # [inline (always)] pub fn read (self) -> (char , Utf8UnreadHandle < 'a , 'b >) { Utf8UnreadHandle :: new_char (self . source) } # [inline (always)] pub fn read_enum (self) -> (Unicode , Utf8UnreadHandle < 'a , 'b >) { Utf8UnreadHandle :: new_enum (self . source) } # [inline (always)] pub fn consumed (& self) -> usize { self . source . consumed () } }
};
}
