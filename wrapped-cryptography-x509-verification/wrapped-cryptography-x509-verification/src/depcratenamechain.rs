// Generated macro for NameChain (struct)
macro_rules! DepcrateNameChain {
() => {
// Module: crate
// Provides: {"NameChain"}
// Dependencies: {}
struct NameChain < 'a , 'chain > { child : Option < & 'a NameChain < 'a , 'chain > > , sans : SubjectAlternativeName < 'chain > , }
};
}
