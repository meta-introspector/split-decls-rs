// Generated macro for decode_spki_spk (function)
macro_rules! Depcrate_verifydecode_spki_spk {
() => {
// Module: crate::verify
// Provides: {"decode_spki_spk"}
// Dependencies: {}
fn decode_spki_spk (spki_spk : & [u8]) -> Result < RsaPublicKey , InvalidSignature > { let mut reader = der :: SliceReader :: new (spki_spk) . map_err (| _ | InvalidSignature) ? ; let ne : [der :: asn1 :: UintRef < '_ > ; 2] = reader . decode () . map_err (| _ | InvalidSignature) ? ; RsaPublicKey :: new (BigUint :: from_bytes_be (ne [0] . as_bytes ()) , BigUint :: from_bytes_be (ne [1] . as_bytes ()) ,) . map_err (| _ | InvalidSignature) }
};
}
