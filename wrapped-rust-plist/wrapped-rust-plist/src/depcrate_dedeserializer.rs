// Generated macro for Deserializer (struct)
macro_rules! Depcrate_deDeserializer {
() => {
// Module: crate::de
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " A structure that deserializes plist event streams into Rust values."] pub struct Deserializer < 'event , I > where I : IntoIterator < Item = Result < Event < 'event > , Error > > , { events : Peekable < < I as IntoIterator > :: IntoIter > , option_mode : OptionMode , in_plist_value : bool , }
};
}
