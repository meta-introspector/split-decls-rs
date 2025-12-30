// Generated macro for test (module)
macro_rules! Depcrate_jointest {
() => {
// Module: crate::join
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_serial_join () { let oper_a = | | 1 + 1 ; let oper_b = | | 2 + 2 ; assert_eq ! ((2 , 4) , SerialJoin :: join (oper_a , oper_b)) ; } # [test] # [cfg (feature = "rayon")] fn test_rayon_join () { let oper_a = | | 1 + 1 ; let oper_b = | | 2 + 2 ; assert_eq ! ((2 , 4) , RayonJoin :: join (oper_a , oper_b)) ; } }
};
}
