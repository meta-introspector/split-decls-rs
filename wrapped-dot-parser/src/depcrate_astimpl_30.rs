// Generated macro for impl_30 (impl)
macro_rules! Depcrate_astimpl_30 {
() => {
// Module: crate::ast
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , A > TryFrom < Pair < 'a , Rule > > for Graph < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inner = p . clone () . into_inner () ; let mut strict = false ; let mut name = None ; let mut pair = inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: strict , Rule :: digraph , Rule :: graph] ,)) ? ; if let Rule :: strict = pair . as_rule () { strict = true ; pair = inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: digraph , Rule :: graph] ,)) ? ; } let is_digraph = match pair . as_rule () { Rule :: digraph => true , Rule :: graph => false , r => { return Err (ParseError :: expect_rule (vec ! [Rule :: digraph , Rule :: graph] , r)) ; } } ; pair = inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident , Rule :: stmt_list] ,)) ? ; if let Rule :: ident = pair . as_rule () { name = Some (String :: from (pair . as_str ())) ; pair = inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: stmt_list])) ? ; } let stmts = StmtList :: try_from (pair) ? ; Ok (Graph { strict , is_digraph , name , stmts , }) } }
};
}
