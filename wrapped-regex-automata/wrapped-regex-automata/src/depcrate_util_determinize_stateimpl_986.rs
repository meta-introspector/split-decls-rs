// Generated macro for impl_986 (impl)
macro_rules! Depcrate_util_determinize_stateimpl_986 {
() => {
// Module: crate::util::determinize::state
// Provides: {"impl_986"}
// Dependencies: {}
# [doc = " This Borrow impl permits us to lookup any state in a map by its byte"] # [doc = " representation. This is particularly convenient when one has a StateBuilder"] # [doc = " and we want to see if a correspondingly equivalent state already exists. If"] # [doc = " one does exist, then we can reuse the allocation required by StateBuilder"] # [doc = " without having to convert it into a State first."] impl core :: borrow :: Borrow < [u8] > for State { fn borrow (& self) -> & [u8] { & self . 0 } }
};
}
