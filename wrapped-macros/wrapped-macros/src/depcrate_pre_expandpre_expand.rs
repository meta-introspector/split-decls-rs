// Generated macro for pre_expand (function)
macro_rules! Depcrate_pre_expandpre_expand {
() => {
// Module: crate::pre_expand
// Provides: {"pre_expand"}
// Dependencies: {}
pub fn pre_expand (from : & Path , to : & Path) { let mut source = String :: new () ; let mut file_from = File :: open (from) . unwrap () ; file_from . read_to_string (& mut source) . unwrap () ; let mut file_to = File :: create (to) . unwrap () ; write_header (& from , & source , & mut file_to) ; let sess = parse :: ParseSess :: new () ; let mut feature_gated_cfgs = Vec :: new () ; let mut cx = ext :: base :: ExtCtxt :: new (& sess , vec ! [] , ext :: expand :: ExpansionConfig :: default ("" . to_owned ()) , & mut feature_gated_cfgs) ; let from = from . to_string_lossy () . into_owned () ; let tts = parse :: parse_tts_from_source_str (from , source , vec ! [] , & sess) ; let tts = find_and_expand_match_token (& mut cx , tts) ; let tts = pretty (& mut cx , tts) ; let expanded = print :: pprust :: tts_to_string (& tts) ; file_to . write_all (expanded . as_bytes ()) . unwrap () ; }
};
}
