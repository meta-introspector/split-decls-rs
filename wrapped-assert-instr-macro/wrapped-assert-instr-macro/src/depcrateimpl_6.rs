// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl syn :: parse :: Parse for Invoc { fn parse (input : syn :: parse :: ParseStream < '_ >) -> syn :: Result < Self > { use syn :: { Token , ext :: IdentExt } ; let mut instr = String :: new () ; while ! input . is_empty () { if input . parse :: < Token ! [,] > () . is_ok () { break ; } if let Ok (ident) = syn :: Ident :: parse_any (input) { instr . push_str (& ident . to_string ()) ; continue ; } if input . parse :: < Token ! [.] > () . is_ok () { instr . push ('.') ; continue ; } if let Ok (s) = input . parse :: < syn :: LitStr > () { instr . push_str (& s . value ()) ; continue ; } println ! ("{:?}" , input . cursor () . token_stream ()) ; return Err (input . error ("expected an instruction")) ; } if instr . is_empty () { return Err (input . error ("expected an instruction before comma")) ; } let mut args = Vec :: new () ; while ! input . is_empty () { let name = input . parse :: < syn :: Ident > () ? ; input . parse :: < Token ! [=] > () ? ; let expr = input . parse :: < syn :: Expr > () ? ; args . push ((name , expr)) ; if input . parse :: < Token ! [,] > () . is_err () { if ! input . is_empty () { return Err (input . error ("extra tokens at end")) ; } break ; } } Ok (Self { instr , args }) } }
};
}
