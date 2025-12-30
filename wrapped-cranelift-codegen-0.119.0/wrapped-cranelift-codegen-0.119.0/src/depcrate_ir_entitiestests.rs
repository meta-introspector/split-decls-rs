// Generated macro for tests (module)
macro_rules! Depcrate_ir_entitiestests {
() => {
// Module: crate::ir::entities
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use alloc :: string :: ToString ; # [test] fn value_with_number () { assert_eq ! (Value :: with_number (0) . unwrap () . to_string () , "v0") ; assert_eq ! (Value :: with_number (1) . unwrap () . to_string () , "v1") ; assert_eq ! (Value :: with_number (u32 :: MAX / 2) , None) ; assert ! (Value :: with_number (u32 :: MAX / 2 - 1) . is_some ()) ; } # [test] fn memory () { use crate :: packed_option :: PackedOption ; use core :: mem ; assert_eq ! (mem :: size_of ::< Value > () , mem :: size_of ::< PackedOption < Value >> ()) ; } # [test] fn memory_option () { use core :: mem ; assert_eq ! (mem :: size_of ::< Value > () * 2 , mem :: size_of ::< Option < Value >> ()) ; } # [test] fn constant_with_number () { assert_eq ! (Constant :: with_number (0) . unwrap () . to_string () , "const0") ; assert_eq ! (Constant :: with_number (1) . unwrap () . to_string () , "const1") ; } }
};
}
