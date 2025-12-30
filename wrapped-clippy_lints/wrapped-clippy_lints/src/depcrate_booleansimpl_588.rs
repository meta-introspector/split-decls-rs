// Generated macro for impl_588 (impl)
macro_rules! Depcrate_booleansimpl_588 {
() => {
// Module: crate::booleans
// Provides: {"impl_588"}
// Dependencies: {}
impl SuggestContext < '_ , '_ , '_ > { fn recurse (& mut self , suggestion : & Bool) -> Option < () > { use quine_mc_cluskey :: Bool :: { And , False , Not , Or , Term , True } ; match suggestion { True => { self . output . push_str ("true") ; } , False => { self . output . push_str ("false") ; } , Not (inner) => match * * inner { And (_) | Or (_) => { self . output . push ('!') ; self . output . push ('(') ; self . recurse (inner) ; self . output . push (')') ; } , Term (n) => { let terminal = self . terminals [n as usize] ; if let Some (str) = simplify_not (self . cx , self . msrv , terminal) { self . output . push_str (& str) ; } else { let mut app = Applicability :: MachineApplicable ; let snip = Sugg :: hir_with_context (self . cx , terminal , SyntaxContext :: root () , "" , & mut app) ; if app != Applicability :: MachineApplicable { return None ; } self . output . push_str (& (! snip) . to_string ()) ; } } , True | False | Not (_) => { self . output . push ('!') ; self . recurse (inner) ? ; } , } , And (v) => { for (index , inner) in v . iter () . enumerate () { if index > 0 { self . output . push_str (" && ") ; } if let Or (_) = * inner { self . output . push ('(') ; self . recurse (inner) ; self . output . push (')') ; } else { self . recurse (inner) ; } } } , Or (v) => { for (index , inner) in v . iter () . rev () . enumerate () { if index > 0 { self . output . push_str (" || ") ; } self . recurse (inner) ; } } , & Term (n) => { self . output . push_str (& self . terminals [n as usize] . span . source_callsite () . get_source_text (self . cx) ? ,) ; } , } Some (()) } }
};
}
