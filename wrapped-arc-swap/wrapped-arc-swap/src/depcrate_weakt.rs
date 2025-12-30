// Generated macro for t (macro)
macro_rules! Depcrate_weakt {
() => {
// Module: crate::weak
// Provides: {"t"}
// Dependencies: {}
macro_rules ! t { ($ name : ident , $ strategy : ty) => { # [cfg (test)] mod $ name { use alloc :: sync :: { Arc , Weak } ; use crate :: ArcSwapAny ; # [allow (deprecated)] type ArcSwapWeak < T > = ArcSwapAny < Weak < T >, $ strategy >; # [test] fn there_and_back () { let data = Arc :: new ("Hello") ; let shared = ArcSwapWeak :: new (Arc :: downgrade (& data)) ; assert_eq ! (1 , Arc :: strong_count (& data)) ; assert_eq ! (1 , Arc :: weak_count (& data)) ; let weak = shared . load () ; assert_eq ! ("Hello" , * weak . upgrade () . unwrap ()) ; assert ! (Arc :: ptr_eq (& data , & weak . upgrade () . unwrap ())) ; } # [test] fn reset () { let data = Arc :: new ("Hello") ; let shared = ArcSwapWeak :: new (Arc :: downgrade (& data)) ; assert_eq ! (1 , Arc :: strong_count (& data)) ; assert_eq ! (1 , Arc :: weak_count (& data)) ; shared . store (Weak :: new ()) ; assert_eq ! (1 , Arc :: strong_count (& data)) ; assert_eq ! (0 , Arc :: weak_count (& data)) ; let weak = shared . load () ; assert ! (weak . upgrade () . is_none ()) ; } # [test] fn destroy () { let data = Arc :: new ("Hello") ; let shared = ArcSwapWeak :: new (Arc :: downgrade (& data)) ; drop (data) ; let weak = shared . load () ; assert ! (weak . upgrade () . is_none ()) ; } } } ; }
};
}
