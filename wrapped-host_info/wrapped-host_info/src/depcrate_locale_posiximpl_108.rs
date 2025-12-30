// Generated macro for impl_108 (impl)
macro_rules! Depcrate_locale_posiximpl_108 {
() => {
// Module: crate::locale::posix
// Provides: {"impl_108"}
// Dependencies: {}
impl < 's > TryFrom < PosixLocale < 's > > for Locale { type Error = ParseError ; fn try_from (input : PosixLocale < 's >) -> Result < Self , Self :: Error > { if input . language == "C" || input . language == "POSIX" { return Ok (locale ! ("en-US-posix")) ; } let mut extensions = Extensions :: new () ; let mut script = None ; let mut variant = None ; let mut language = Language :: try_from_str (input . language) ? ; let region = input . territory . map (Region :: try_from_str) . transpose () ? ; if let Some (modifier) = input . modifier { match modifier . to_ascii_lowercase () . as_str () { "euro" => { extensions . unicode . keywords . set (key ! ("cu") , value ! ("eur")) ; } "cyrillic" => script = Some (script ! ("Cyrl")) , "devanagari" => script = Some (script ! ("Deva")) , "latin" => script = Some (script ! ("Latn")) , "saaho" => language = language ! ("ssy") , "valencia" => variant = Some (variant ! ("valencia")) , _ => () , } } Ok (Locale { id : LanguageIdentifier { language , region , script , variants : variant . map_or_else (Variants :: new , Variants :: from_variant) , } , extensions , }) } }
};
}
