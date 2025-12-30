// Generated macro for tests (module)
macro_rules! Depcrate_basetests {
() => {
// Module: crate::base
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: boolean :: CFBoolean ; use std :: mem ; # [test] fn cftype_instance_of () { let string = CFString :: from_static_string ("foo") ; let cftype = string . as_CFType () ; assert ! (cftype . instance_of ::< CFString > ()) ; assert ! (! cftype . instance_of ::< CFBoolean > ()) ; } # [test] fn as_cftype_retain_count () { let string = CFString :: from_static_string ("alongerstring") ; assert_eq ! (string . retain_count () , 1) ; let cftype = string . as_CFType () ; assert_eq ! (cftype . retain_count () , 2) ; mem :: drop (string) ; assert_eq ! (cftype . retain_count () , 1) ; } # [test] fn into_cftype_retain_count () { let string = CFString :: from_static_string ("alongerstring") ; assert_eq ! (string . retain_count () , 1) ; let cftype = string . into_CFType () ; assert_eq ! (cftype . retain_count () , 1) ; } # [test] fn as_cftype_and_downcast () { let string = CFString :: from_static_string ("alongerstring") ; let cftype = string . as_CFType () ; let string2 = cftype . downcast :: < CFString > () . unwrap () ; assert_eq ! (string2 . to_string () , "alongerstring") ; assert_eq ! (string . retain_count () , 3) ; assert_eq ! (cftype . retain_count () , 3) ; assert_eq ! (string2 . retain_count () , 3) ; } # [test] fn into_cftype_and_downcast_into () { let string = CFString :: from_static_string ("alongerstring") ; let cftype = string . into_CFType () ; let string2 = cftype . downcast_into :: < CFString > () . unwrap () ; assert_eq ! (string2 . to_string () , "alongerstring") ; assert_eq ! (string2 . retain_count () , 1) ; } }
};
}
