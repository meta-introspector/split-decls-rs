// Generated macro for test (module)
macro_rules! Depcrate_optionstest {
() => {
// Module: crate::options
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] pub mod test { use crate :: options :: parser :: { Arg , MatchedFlags } ; use std :: ffi :: OsStr ; # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum Strictnesses { Last , Complain , Both , } # [doc = " This function gets used by the other testing modules."] # [doc = " It can run with one or both strictness values: if told to run with"] # [doc = " both, then both should resolve to the same result."] # [doc = ""] # [doc = " It returns a vector with one or two elements in."] # [doc = " These elements can then be tested with `assert_eq` or what have you."] pub fn parse_for_test < T , F > (inputs : & [& str] , args : & 'static [& 'static Arg] , strictnesses : Strictnesses , get : F ,) -> Vec < T > where F : Fn (& MatchedFlags < '_ >) -> T , { use self :: Strictnesses :: * ; use crate :: options :: parser :: { Args , Strictness } ; let bits = inputs . iter () . map (OsStr :: new) . collect :: < Vec < _ > > () ; let mut result = Vec :: new () ; if strictnesses == Last || strictnesses == Both { let results = Args (args) . parse (bits . clone () , Strictness :: UseLastArguments) ; result . push (get (& results . unwrap () . flags)) ; } if strictnesses == Complain || strictnesses == Both { let results = Args (args) . parse (bits , Strictness :: ComplainAboutRedundantArguments) ; result . push (get (& results . unwrap () . flags)) ; } result } }
};
}
