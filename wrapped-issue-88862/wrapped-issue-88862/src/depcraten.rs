// Generated macro for N (trait)
macro_rules! DepcrateN {
() => {
// Module: crate
// Provides: {"N"}
// Dependencies: {}
trait N < T , R > : 'static where R : std :: future :: Future , R :: Output : Responder , { fn call (& self , param : T) -> R ; }
};
}
