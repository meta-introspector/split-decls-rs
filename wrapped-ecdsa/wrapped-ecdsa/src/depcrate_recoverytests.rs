// Generated macro for tests (module)
macro_rules! Depcrate_recoverytests {
() => {
// Module: crate::recovery
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: RecoveryId ; # [test] fn new () { assert_eq ! (RecoveryId :: new (false , false) . to_byte () , 0) ; assert_eq ! (RecoveryId :: new (true , false) . to_byte () , 1) ; assert_eq ! (RecoveryId :: new (false , true) . to_byte () , 2) ; assert_eq ! (RecoveryId :: new (true , true) . to_byte () , 3) ; } # [test] fn try_from () { for n in 0u8 ..= 3 { assert_eq ! (RecoveryId :: try_from (n) . expect ("RecoveryId") . to_byte () , n) ; } for n in 4u8 ..= 255 { assert ! (RecoveryId :: try_from (n) . is_err ()) ; } } # [test] fn is_x_reduced () { assert ! (! RecoveryId :: try_from (0) . expect ("RecoveryId") . is_x_reduced ()) ; assert ! (! RecoveryId :: try_from (1) . expect ("RecoveryId") . is_x_reduced ()) ; assert ! (RecoveryId :: try_from (2) . expect ("RecoveryId") . is_x_reduced ()) ; assert ! (RecoveryId :: try_from (3) . expect ("RecoveryId") . is_x_reduced ()) ; } # [test] fn is_y_odd () { assert ! (! RecoveryId :: try_from (0) . expect ("RecoveryId") . is_y_odd ()) ; assert ! (RecoveryId :: try_from (1) . expect ("RecoveryId") . is_y_odd ()) ; assert ! (! RecoveryId :: try_from (2) . expect ("RecoveryId") . is_y_odd ()) ; assert ! (RecoveryId :: try_from (3) . expect ("RecoveryId") . is_y_odd ()) ; } }
};
}
