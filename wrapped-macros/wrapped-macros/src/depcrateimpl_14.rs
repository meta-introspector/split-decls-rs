// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Parse for Input { fn parse (input : ParseStream) -> parse :: Result < Self > { let formatter = input . parse () ? ; let _comma = input . parse () ? ; let literal = input . parse () ? ; if input . is_empty () { Ok (Input { formatter , _comma , literal , _comma2 : None , args : Punctuated :: new () , }) } else { Ok (Input { formatter , _comma , literal , _comma2 : input . parse () ? , args : Punctuated :: parse_terminated (input) ? , }) } } }
};
}
