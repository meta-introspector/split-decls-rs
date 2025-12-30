// Generated macro for message (function)
macro_rules! Depcrate_tag_decodemessage {
() => {
// Module: crate::tag::decode
// Provides: {"message"}
// Dependencies: {}
pub fn message < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < (& 'a BStr , Option < & 'a BStr >) , E > { const PGP_SIGNATURE_BEGIN : & [u8] = b"\n-----BEGIN PGP SIGNATURE-----" ; const PGP_SIGNATURE_END : & [u8] = b"-----END PGP SIGNATURE-----" ; if i . iter () . all (| b | * b == b'\n') { return i . map (| message : & [u8] | (message . as_bstr () , None)) . parse_next (i) ; } delimited (NL , alt (((take_until (0 .. , PGP_SIGNATURE_BEGIN) , preceded (NL , (& PGP_SIGNATURE_BEGIN [1 ..] , take_until (0 .. , PGP_SIGNATURE_END) , PGP_SIGNATURE_END , rest ,) . take () . map (| signature : & [u8] | { if signature . is_empty () { None } else { Some (signature . as_bstr ()) } }) ,) ,) , rest . map (| rest : & [u8] | (rest , None)) ,)) , opt (NL) ,) . map (| (message , signature) | (message . as_bstr () , signature)) . parse_next (i) }
};
}
