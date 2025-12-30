// Generated macro for signature (function)
macro_rules! Depcrate_parsesignature {
() => {
// Module: crate::parse
// Provides: {"signature"}
// Dependencies: {}
pub (crate) fn signature < 'a , E : ParserError < & 'a [u8] > + AddContext < & 'a [u8] , StrContext > > (i : & mut & 'a [u8] ,) -> ModalResult < gix_actor :: SignatureRef < 'a > , E > { gix_actor :: signature :: decode (i) }
};
}
