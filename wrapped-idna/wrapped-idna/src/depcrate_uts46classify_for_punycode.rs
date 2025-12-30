// Generated macro for classify_for_punycode (function)
macro_rules! Depcrate_uts46classify_for_punycode {
() => {
// Module: crate::uts46
// Provides: {"classify_for_punycode"}
// Dependencies: {}
# [inline (always)] fn classify_for_punycode (label : & [char]) -> PunycodeClassification { let mut iter = label . iter () . copied () ; loop { if let Some (c) = iter . next () { if c . is_ascii () { continue ; } if c == '\u{FFFD}' { return PunycodeClassification :: Error ; } for c in iter { if c == '\u{FFFD}' { return PunycodeClassification :: Error ; } } return PunycodeClassification :: Unicode ; } return PunycodeClassification :: Ascii ; } }
};
}
