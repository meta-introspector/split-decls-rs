// Generated macro for expand (function)
macro_rules! Depcrate_cargoexpand {
() => {
// Module: crate::cargo
// Provides: {"expand"}
// Dependencies: {}
pub (crate) fn expand < I , S > (project : & Project , name : & Name , args : & Option < I > ,) -> Result < (bool , Vec < u8 >) > where I : IntoIterator < Item = S > + Clone , S : AsRef < OsStr > , { let mut cargo = cargo (project) ; let cargo = cargo . arg ("expand") . arg ("--bin") . arg (name . as_ref ()) . arg ("--theme") . arg ("none") ; if let Some (args) = args { cargo . args (args . clone ()) ; } let cargo_expand = cargo . output () . map_err (| e | Error :: CargoExpandExecution (e . to_string ())) ? ; if ! cargo_expand . status . success () { return Ok ((false , cargo_expand . stderr)) ; } Ok ((true , cargo_expand . stdout)) }
};
}
