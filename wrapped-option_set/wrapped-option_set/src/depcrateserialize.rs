// Generated macro for serialize (function)
macro_rules! Depcrateserialize {
() => {
// Module: crate
// Provides: {"serialize"}
// Dependencies: {}
# [doc = " Serialize an OptionSet's set bits as a sequence of strings."] pub fn serialize < T , S > (options : & T , serializer : S , transform : CaseTransform ,) -> Result < S :: Ok , S :: Error > where T : OptionSet , S : Serializer , { assert ! (T :: VARIANTS . len () == T :: NAMES . len ()) ; let mut seq = serializer . serialize_seq (T :: NAMES . len () . into ()) ? ; for (& variant , & name) in T :: VARIANTS . iter () . zip (T :: NAMES) { if * options & variant == variant { seq . serialize_element (& transform . apply (name)) ? ; } } seq . end () }
};
}
