// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl < T , E : Into < Diagnostic > > ResultExt for Result < T , E > { type Ok = T ; fn unwrap_or_abort (self) -> T { match self { Ok (res) => res , Err (e) => e . into () . abort () , } } fn expect_or_abort (self , message : & str) -> T { match self { Ok (res) => res , Err (e) => { let mut e = e . into () ; e . msg = format ! ("{}: {}" , message , e . msg) ; e . abort () } } } }
};
}
