// Generated macro for tests (module)
macro_rules! Depcrate_idtests {
() => {
// Module: crate::id
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn standard_id_new () { assert_eq ! (StandardId :: new (StandardId :: MAX . as_raw ()) , Some (StandardId :: MAX)) ; } # [test] fn standard_id_new_out_of_range () { assert_eq ! (StandardId :: new (StandardId :: MAX . as_raw () + 1) , None) ; } # [test] fn standard_id_new_unchecked_out_of_range () { let id = StandardId :: MAX . as_raw () + 1 ; assert_eq ! (unsafe { StandardId :: new_unchecked (id) } , StandardId (id)) ; } # [test] fn extended_id_new () { assert_eq ! (ExtendedId :: new (ExtendedId :: MAX . as_raw ()) , Some (ExtendedId :: MAX)) ; } # [test] fn extended_id_new_out_of_range () { assert_eq ! (ExtendedId :: new (ExtendedId :: MAX . as_raw () + 1) , None) ; } # [test] fn extended_id_new_unchecked_out_of_range () { let id = ExtendedId :: MAX . as_raw () + 1 ; assert_eq ! (unsafe { ExtendedId :: new_unchecked (id) } , ExtendedId (id)) ; } # [test] fn get_standard_id_from_extended_id () { assert_eq ! (Some (ExtendedId :: MAX . standard_id ()) , StandardId :: new ((ExtendedId :: MAX . 0 >> 18) as u16)) ; } # [test] fn cmp_id () { assert ! (StandardId :: ZERO < StandardId :: MAX) ; assert ! (ExtendedId :: ZERO < ExtendedId :: MAX) ; assert ! (Id :: Standard (StandardId :: ZERO) < Id :: Extended (ExtendedId :: ZERO)) ; assert ! (Id :: Extended (ExtendedId :: ZERO) < Id :: Extended (ExtendedId :: MAX)) ; assert ! (Id :: Extended (ExtendedId ((1 << 11) - 1)) < Id :: Standard (StandardId (1))) ; assert ! (Id :: Standard (StandardId (1)) < Id :: Extended (ExtendedId :: MAX)) ; } }
};
}
