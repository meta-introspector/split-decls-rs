// Generated macro for ExemplarCharactersBorrowed (struct)
macro_rules! Depcrate_exemplar_charsExemplarCharactersBorrowed {
() => {
// Module: crate::exemplar_chars
// Provides: {"ExemplarCharactersBorrowed"}
// Dependencies: {}
# [doc = " A borrowed wrapper around code point set data, returned by"] # [doc = " [`ExemplarCharacters::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct ExemplarCharactersBorrowed < 'a > { data : & 'a ExemplarCharactersData < 'a > , }
};
}
