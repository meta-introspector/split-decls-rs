// Generated macro for indicator (module)
macro_rules! Depcrate_fipsindicator {
() => {
// Module: crate::fips
// Provides: {"indicator"}
// Dependencies: {}
# [cfg (all (feature = "fips" , debug_assertions))] pub (crate) mod indicator { use core :: cell :: Cell ; thread_local ! { static STATUS_INDICATOR : Cell < Option < bool >> = const { Cell :: new (None) } ; } pub fn get_status () -> Option < bool > { STATUS_INDICATOR . with (| v | { let swap = Cell :: new (None) ; v . swap (& swap) ; swap . take () }) } pub fn set_approved () { STATUS_INDICATOR . with (| v | v . set (Some (true))) ; } pub fn set_unapproved () { STATUS_INDICATOR . with (| v | v . set (Some (false))) ; } pub fn clear () { STATUS_INDICATOR . with (| v | v . set (None)) ; } }
};
}
