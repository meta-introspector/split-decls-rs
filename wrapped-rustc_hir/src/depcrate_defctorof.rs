// Generated macro for CtorOf (enum)
macro_rules! Depcrate_defCtorOf {
() => {
// Module: crate::def
// Provides: {"CtorOf"}
// Dependencies: {}
# [doc = " Encodes if a `DefKind::Ctor` is the constructor of an enum variant or a struct."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum CtorOf { # [doc = " This `DefKind::Ctor` is a synthesized constructor of a tuple or unit struct."] Struct , # [doc = " This `DefKind::Ctor` is a synthesized constructor of a tuple or unit variant."] Variant , }
};
}
