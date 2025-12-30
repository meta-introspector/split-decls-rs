// Generated macro for Clock (struct)
macro_rules! DepcrateClock {
() => {
// Module: crate
// Provides: {"Clock"}
// Dependencies: {}
# [doc = " A representation of network time."] # [doc = ""] # [doc = " All members of `Clock` start from 0 upon network boot."] # [repr (C)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Debug , CloneZeroed , Default , PartialEq , Eq)] pub struct Clock { # [doc = " The current `Slot`."] pub slot : Slot , # [doc = " The timestamp of the first `Slot` in this `Epoch`."] pub epoch_start_timestamp : UnixTimestamp , # [doc = " The current `Epoch`."] pub epoch : Epoch , # [doc = " The future `Epoch` for which the leader schedule has"] # [doc = " most recently been calculated."] pub leader_schedule_epoch : Epoch , # [doc = " The approximate real world time of the current slot."] # [doc = ""] # [doc = " This value was originally computed from genesis creation time and"] # [doc = " network time in slots, incurring a lot of drift. Following activation of"] # [doc = " the [`timestamp_correction` and `timestamp_bounding`][tsc] features it"] # [doc = " is calculated using a [validator timestamp oracle][oracle]."] # [doc = ""] # [doc = " [tsc]: https://docs.solanalabs.com/implemented-proposals/bank-timestamp-correction"] # [doc = " [oracle]: https://docs.solanalabs.com/implemented-proposals/validator-timestamp-oracle"] pub unix_timestamp : UnixTimestamp , }
};
}
