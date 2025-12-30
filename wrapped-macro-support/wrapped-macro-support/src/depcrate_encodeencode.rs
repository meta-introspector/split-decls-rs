// Generated macro for encode (function)
macro_rules! Depcrate_encodeencode {
() => {
// Module: crate::encode
// Provides: {"encode"}
// Dependencies: {}
pub fn encode (program : & ast :: Program) -> Result < EncodeResult , Diagnostic > { let mut e = Encoder :: new () ; let i = Interner :: new () ; shared_program (program , & i) ? . encode (& mut e) ; let custom_section = e . finish () ; let included_files = i . files . borrow () . values () . map (| p | & p . path) . cloned () . collect () ; Ok (EncodeResult { custom_section , included_files , }) }
};
}
