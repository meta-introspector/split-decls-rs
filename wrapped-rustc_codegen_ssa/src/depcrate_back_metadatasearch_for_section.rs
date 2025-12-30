// Generated macro for search_for_section (function)
macro_rules! Depcrate_back_metadatasearch_for_section {
() => {
// Module: crate::back::metadata
// Provides: {"search_for_section"}
// Dependencies: {}
pub (super) fn search_for_section < 'a > (path : & Path , bytes : & 'a [u8] , section : & str ,) -> Result < & 'a [u8] , String > { let Ok (file) = object :: File :: parse (bytes) else { return Ok (bytes) ; } ; file . section_by_name (section) . ok_or_else (| | format ! ("no `{}` section in '{}'" , section , path . display ())) ? . data () . map_err (| e | format ! ("failed to read {} section in '{}': {}" , section , path . display () , e)) }
};
}
