// Generated macro for impl_93 (impl)
macro_rules! Depcrate_iterators_pairimpl_93 {
() => {
// Module: crate::iterators::pair
// Provides: {"impl_93"}
// Dependencies: {}
impl < R : RuleType > fmt :: Display for Pair < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let rule = self . as_rule () ; let start = self . pos (self . start) ; let end = self . pos (self . pair ()) ; let mut pairs = self . clone () . into_inner () . peekable () ; if pairs . peek () . is_none () { write ! (f , "{:?}({}, {})" , rule , start , end) } else { write ! (f , "{:?}({}, {}, [{}])" , rule , start , end , pairs . map (| pair | format ! ("{}" , pair)) . collect ::< Vec < _ >> () . join (", ")) } } }
};
}
