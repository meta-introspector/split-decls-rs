// Generated macro for UNICODE_PROPERTY_NAMES (static)
macro_rules! DepcrateUNICODE_PROPERTY_NAMES {
() => {
// Module: crate
// Provides: {"UNICODE_PROPERTY_NAMES"}
// Dependencies: {}
# [doc (hidden)] # [deprecated (note = "use `pest::unicode::unicode_property_names` instead")] pub static UNICODE_PROPERTY_NAMES : LazyLock < Vec < & str > > = LazyLock :: new (| | unicode_property_names () . collect :: < Vec < _ > > ()) ;
};
}
