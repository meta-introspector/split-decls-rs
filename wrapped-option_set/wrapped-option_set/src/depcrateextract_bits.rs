// Generated macro for extract_bits (function)
macro_rules! Depcrateextract_bits {
() => {
// Module: crate
// Provides: {"extract_bits"}
// Dependencies: {}
# [doc = " Actually performs the sequence processing and flag extraction."] fn extract_bits < 'de , A , T , S > (mut seq : A , names : & [S]) -> Result < T , A :: Error > where A : SeqAccess < 'de > , T : OptionSet , S : Deref < Target = str > , { use serde :: de :: Error ; let mut flags = T :: default () ; while let Some (elem) = seq . next_element :: < Str < 'de > > () ? { let mut iter = T :: VARIANTS . iter () . zip (names) ; match iter . find (| & (_ , name) | * * name == * elem) { Some ((& flag , _)) => flags |= flag , None => Err (A :: Error :: unknown_variant (& elem , T :: NAMES)) ? , } } Ok (flags) }
};
}
