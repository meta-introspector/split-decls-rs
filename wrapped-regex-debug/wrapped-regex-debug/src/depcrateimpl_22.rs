// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl Args { fn parse_one (& self) -> Result < Expr > { parse (& self . arg_pattern) } fn parse_many (& self) -> Result < Vec < Expr > > { self . arg_patterns . iter () . map (| s | parse (s)) . collect () } fn literals < F : Fn (& mut Literals , & Expr) -> bool > (& self , exprs : & [Expr] , get_literals : F ,) -> Literals { let mut lits = Some (self . empty_literals ()) ; for e in exprs { lits = lits . and_then (| mut lits | { if ! get_literals (& mut lits , e) { None } else { Some (lits) } }) ; } lits . unwrap_or (self . empty_literals ()) } fn empty_literals (& self) -> Literals { let mut lits = Literals :: empty () ; lits . set_limit_size (self . flag_literal_limit) ; lits . set_limit_class (self . flag_class_limit) ; lits } fn compiler (& self) -> Compiler { Compiler :: new () . size_limit (self . flag_size_limit) } }
};
}
