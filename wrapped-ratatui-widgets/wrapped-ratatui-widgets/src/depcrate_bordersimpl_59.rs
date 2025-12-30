// Generated macro for impl_59 (impl)
macro_rules! Depcrate_bordersimpl_59 {
() => {
// Module: crate::borders
// Provides: {"impl_59"}
// Dependencies: {}
impl fmt :: Debug for Borders { # [doc = " Display the Borders bitflags as a list of names."] # [doc = ""] # [doc = " `Borders::NONE` is displayed as `NONE` and `Borders::ALL` is displayed as `ALL`. If multiple"] # [doc = " flags are set, they are otherwise displayed separated by a pipe character."] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_empty () { return write ! (f , "NONE") ; } if self . is_all () { return write ! (f , "ALL") ; } let mut names = self . iter_names () . map (| (name , _) | name) ; if let Some (first) = names . next () { write ! (f , "{first}") ? ; } for name in names { write ! (f , " | {name}") ? ; } Ok (()) } }
};
}
