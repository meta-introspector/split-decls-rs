// Generated macro for deserialize (function)
macro_rules! Depcratedeserialize {
() => {
// Module: crate
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Deserialize set bits from a sequence of name strings."] pub fn deserialize < 'de , T , D > (deserializer : D , transform : CaseTransform) -> Result < T , D :: Error > where T : OptionSet , D : Deserializer < 'de > , { deserializer . deserialize_seq (OptionSetVisitor (transform , PhantomData)) }
};
}
