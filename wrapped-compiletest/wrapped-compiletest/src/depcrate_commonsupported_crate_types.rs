// Generated macro for supported_crate_types (function)
macro_rules! Depcrate_commonsupported_crate_types {
() => {
// Module: crate::common
// Provides: {"supported_crate_types"}
// Dependencies: {}
fn supported_crate_types (config : & Config) -> HashSet < String > { let crate_types : HashSet < _ > = query_rustc_output (config , & ["--target" , & config . target , "--print=supported-crate-types" , "-Zunstable-options"] , Default :: default () ,) . lines () . map (| l | l . to_string ()) . collect () ; for crate_type in crate_types . iter () { assert ! (KNOWN_CRATE_TYPES . contains (& crate_type . as_str ()) , "unexpected crate type `{}`: known crate types are {:?}" , crate_type , KNOWN_CRATE_TYPES) ; } crate_types }
};
}
