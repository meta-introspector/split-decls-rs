// Generated macro for CharacterAndClass (struct)
macro_rules! DepcrateCharacterAndClass {
() => {
// Module: crate
// Provides: {"CharacterAndClass"}
// Dependencies: {}
# [doc = " Pack a `char` and a `CanonicalCombiningClass` in"] # [doc = " 32 bits (the former in the lower 24 bits and the"] # [doc = " latter in the high 8 bits). The latter can be"] # [doc = " initialized to 0xFF upon creation, in which case"] # [doc = " it can be actually set later by calling"] # [doc = " `set_ccc_from_trie_if_not_already_set`. This is"] # [doc = " a micro optimization to avoid the Canonical"] # [doc = " Combining Class trie lookup when there is only"] # [doc = " one combining character in a sequence. This type"] # [doc = " is intentionally non-`Copy` to get compiler help"] # [doc = " in making sure that the class is set on the"] # [doc = " instance on which it is intended to be set"] # [doc = " and not on a temporary copy."] # [doc = ""] # [doc = " Note that 0xFF is won't be assigned to an actual"] # [doc = " canonical combining class per definition D104"] # [doc = " in The Unicode Standard."] # [derive (Debug)] struct CharacterAndClass (u32) ;
};
}
