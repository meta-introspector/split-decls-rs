// Generated macro for tests (module)
macro_rules! Depcrate_cal_coptictests {
() => {
// Module: crate::cal::coptic
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: options :: { DateFromFieldsOptions , MissingFieldsStrategy , Overflow } ; use crate :: types :: { DateFields , Month } ; # [test] fn test_coptic_regression () { let iso_date = Date :: try_new_iso (- 100 , 3 , 3) . unwrap () ; let coptic = iso_date . to_calendar (Coptic) ; let recovered_iso = coptic . to_iso () ; assert_eq ! (iso_date , recovered_iso) ; } # [test] fn test_from_fields_monthday_constrain () { let month = Month :: new (13) . code () ; let fields = DateFields { month_code : Some (month . 0 . as_bytes ()) , day : Some (7) , .. Default :: default () } ; let options = DateFromFieldsOptions { overflow : Some (Overflow :: Constrain) , missing_fields_strategy : Some (MissingFieldsStrategy :: Ecma) , .. Default :: default () } ; let date = Date :: try_from_fields (fields , options , Coptic) . unwrap () ; assert_eq ! (date . day_of_month () . 0 , 6 , "Day was successfully constrained") ; } }
};
}
