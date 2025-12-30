// Generated macro for tests (module)
macro_rules! Depcrate_numbertests {
() => {
// Module: crate::number
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn to_from_bool () { let cffalse = CFBoolean :: new (false) ; let cftrue = CFBoolean :: new (true) ; assert_ne ! (cffalse , cftrue) ; assert_eq ! (cftrue , CFBoolean :: new (true)) ; assert_eq ! (cffalse , CFBoolean :: new (false)) ; assert ! (! cffalse . as_bool ()) ; assert ! (cftrue . as_bool ()) ; } # [test] fn to_from_number () { let n = CFNumber :: new_i32 (442) ; if cfg ! (all (target_os = "macos" , target_arch = "x86")) { assert_eq ! (n . as_i8 () , None) ; } else { assert_eq ! (n . as_i8 () , Some (442i32 as i8)) ; } assert_eq ! (n . as_i16 () , Some (442)) ; assert_eq ! (n . as_i32 () , Some (442)) ; assert_eq ! (n . as_i64 () , Some (442)) ; assert_eq ! (n . as_f32 () , Some (442.0)) ; assert_eq ! (n . as_f64 () , Some (442.0)) ; } # [test] fn cmp_number () { assert ! (CFNumber :: new_i32 (2) < CFNumber :: new_i32 (3)) ; assert ! (CFNumber :: new_i32 (3) == CFNumber :: new_i32 (3)) ; assert ! (CFNumber :: new_i32 (4) > CFNumber :: new_i32 (3)) ; } }
};
}
