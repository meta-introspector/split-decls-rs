// Generated macro for tests (module)
macro_rules! Depcrate_cachetests {
() => {
// Module: crate::cache
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use alloc :: sync :: Arc ; use super :: * ; use crate :: { ArcSwap , ArcSwapOption } ; # [test] fn cached_value () { let a = ArcSwap :: from_pointee (42) ; let mut c1 = Cache :: new (& a) ; let mut c2 = Cache :: new (& a) ; assert_eq ! (42 , ** c1 . load ()) ; assert_eq ! (42 , ** c2 . load ()) ; a . store (Arc :: new (43)) ; assert_eq ! (42 , ** c1 . load_no_revalidate ()) ; assert_eq ! (43 , ** c1 . load ()) ; } # [test] fn cached_through_arc () { let a = Arc :: new (ArcSwap :: from_pointee (42)) ; let mut c = Cache :: new (Arc :: clone (& a)) ; assert_eq ! (42 , ** c . load ()) ; a . store (Arc :: new (0)) ; drop (a) ; } # [test] fn cache_option () { let a = ArcSwapOption :: from_pointee (42) ; let mut c = Cache :: new (& a) ; assert_eq ! (42 , ** c . load () . as_ref () . unwrap ()) ; a . store (None) ; assert ! (c . load () . is_none ()) ; } struct Inner { answer : usize , } struct Outer { inner : Inner , } # [test] fn map_cache () { let a = ArcSwap :: from_pointee (Outer { inner : Inner { answer : 42 } , }) ; let mut cache = Cache :: new (& a) ; let mut inner = cache . clone () . map (| outer | & outer . inner) ; let mut answer = cache . clone () . map (| outer | & outer . inner . answer) ; assert_eq ! (42 , cache . load () . inner . answer) ; assert_eq ! (42 , inner . load () . answer) ; assert_eq ! (42 , * answer . load ()) ; a . store (Arc :: new (Outer { inner : Inner { answer : 24 } , })) ; assert_eq ! (24 , cache . load () . inner . answer) ; assert_eq ! (24 , inner . load () . answer) ; assert_eq ! (24 , * answer . load ()) ; } }
};
}
