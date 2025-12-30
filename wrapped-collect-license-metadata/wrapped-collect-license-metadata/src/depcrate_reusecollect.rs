// Generated macro for collect (function)
macro_rules! Depcrate_reusecollect {
() => {
// Module: crate::reuse
// Provides: {"collect"}
// Dependencies: {}
pub (crate) fn collect (reuse_exe : & Path , interner : & mut LicensesInterner ,) -> Result < Vec < (PathBuf , LicenseId) > , Error > { println ! ("gathering license information from REUSE (this might take a minute...)") ; let start = Instant :: now () ; let raw = & obtain_spdx_document (reuse_exe) ? ; println ! ("finished gathering the license information from REUSE in {:.2?}" , start . elapsed ()) ; let document = spdx_rs :: parsers :: spdx_from_tag_value (& raw) ? ; let mut result = Vec :: new () ; for file in document . file_information { let concluded_license = file . concluded_license . expect ("File should have licence info") ; let copyright_text = file . copyright_text . expect ("File should have copyright text") ; let license = interner . intern (License { spdx : concluded_license . to_string () , copyright : copyright_text . split ('\n') . map (| s | s . into ()) . collect () , }) ; result . push ((file . file_name . into () , license)) ; } Ok (result) }
};
}
