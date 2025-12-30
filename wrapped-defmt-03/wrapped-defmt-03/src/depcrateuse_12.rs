// Generated macro for use_12 (pub_use)
macro_rules! Depcrateuse_12 {
() => {
// Module: crate
// Provides: {"use_12"}
// Dependencies: {}
# [doc = " This is an alias for defmt's [`unwrap`] macro which supports messages like std's except."] # [doc = " ```"] # [doc = " use defmt::expect;"] # [doc = ""] # [doc = " # let result = Ok::<(), ()>(());"] # [doc = " # let arg = ();"] # [doc = " let x = result.expect(&format!(\"text {:?}\", arg));"] # [doc = " let x = expect!(result, \"text {:?}\", arg); // arg must be implement `Format`"] # [doc = " ```"] # [doc = ""] # [doc = " For the complete documentation see that of defmt's *unwrap* macro."] pub use defmt10 :: unwrap as expect ;
};
}
