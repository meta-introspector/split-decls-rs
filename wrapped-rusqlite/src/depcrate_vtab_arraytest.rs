// Generated macro for test (module)
macro_rules! Depcrate_vtab_arraytest {
() => {
// Module: crate::vtab::array
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: types :: Value ; use crate :: vtab :: array ; use crate :: { Connection , Result } ; use std :: rc :: Rc ; # [test] fn test_array_module () -> Result < () > { let db = Connection :: open_in_memory () ? ; array :: load_module (& db) ? ; let v = vec ! [1i64 , 2 , 3 , 4] ; let values : Vec < Value > = v . into_iter () . map (Value :: from) . collect () ; let ptr = Rc :: new (values) ; { let mut stmt = db . prepare ("SELECT value from rarray(?1);") ? ; let rows = stmt . query_map ([& ptr] , | row | row . get :: < _ , i64 > (0)) ? ; assert_eq ! (2 , Rc :: strong_count (& ptr)) ; let mut count = 0 ; for (i , value) in rows . enumerate () { assert_eq ! (i as i64 , value ? - 1) ; count += 1 ; } assert_eq ! (4 , count) ; } assert_eq ! (1 , Rc :: strong_count (& ptr)) ; Ok (()) } }
};
}
