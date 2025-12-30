// Generated macro for impl_27 (impl)
macro_rules! Depcrate_selectimpl_27 {
() => {
// Module: crate::select
// Provides: {"impl_27"}
// Dependencies: {}
impl Parse for Select { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let mut select = Self { complete : None , default : None , normal_fut_exprs : vec ! [] , normal_fut_handlers : vec ! [] , } ; while ! input . is_empty () { let case_kind = if input . peek (kw :: complete) { if select . complete . is_some () { return Err (input . error ("multiple `complete` cases found, only one allowed")) ; } input . parse :: < kw :: complete > () ? ; CaseKind :: Complete } else if input . peek (Token ! [default]) { if select . default . is_some () { return Err (input . error ("multiple `default` cases found, only one allowed")) ; } input . parse :: < Ident > () ? ; CaseKind :: Default } else { let pat = Pat :: parse_multi_with_leading_vert (input) ? ; input . parse :: < Token ! [=] > () ? ; let expr = input . parse () ? ; CaseKind :: Normal (pat , expr) } ; input . parse :: < Token ! [=>] > () ? ; let expr = Expr :: parse_with_earlier_boundary_rule (input) ? ; let is_block = matches ! (expr , Expr :: Block (_)) ; if is_block || input . is_empty () { input . parse :: < Option < Token ! [,] > > () ? ; } else { input . parse :: < Token ! [,] > () ? ; } match case_kind { CaseKind :: Complete => select . complete = Some (expr) , CaseKind :: Default => select . default = Some (expr) , CaseKind :: Normal (pat , fut_expr) => { select . normal_fut_exprs . push (fut_expr) ; select . normal_fut_handlers . push ((pat , expr)) ; } } } Ok (select) } }
};
}
