// Generated macro for processing (function)
macro_rules! Depcrate_uts46processing {
() => {
// Module: crate::uts46
// Provides: {"processing"}
// Dependencies: {}
# [doc = " http://www.unicode.org/reports/tr46/#Processing"] fn processing (domain : & str , flags : Flags , errors : & mut Vec < Error >) -> String { let mut mapped = String :: new () ; for c in domain . chars () { map_char (c , flags , & mut mapped , errors) } let normalized : String = mapped . nfc () . collect () ; let mut is_bidi_domain = domain . chars () . any (| c | matches ! (bidi_class (c) , BidiClass :: R | BidiClass :: AL | BidiClass :: AN)) ; if ! is_bidi_domain { for label in normalized . split ('.') { if label . starts_with (PUNYCODE_PREFIX) { match punycode :: decode_to_string (& label [PUNYCODE_PREFIX . len () ..]) { Some (decoded_label) => { if decoded_label . chars () . any (| c | matches ! (bidi_class (c) , BidiClass :: R | BidiClass :: AL | BidiClass :: AN)) { is_bidi_domain = true ; } } None => { is_bidi_domain = true ; } } } } } let mut validated = String :: new () ; let mut first = true ; for label in normalized . split ('.') { if ! first { validated . push ('.') ; } first = false ; if label . starts_with (PUNYCODE_PREFIX) { match punycode :: decode_to_string (& label [PUNYCODE_PREFIX . len () ..]) { Some (decoded_label) => { let flags = Flags { transitional_processing : false , .. flags } ; validate (& decoded_label , is_bidi_domain , flags , errors) ; validated . push_str (& decoded_label) } None => errors . push (Error :: PunycodeError) } } else { validate (label , is_bidi_domain , flags , errors) ; validated . push_str (label) } } validated }
};
}
