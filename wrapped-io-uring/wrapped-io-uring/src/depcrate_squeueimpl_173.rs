// Generated macro for impl_173 (impl)
macro_rules! Depcrate_squeueimpl_173 {
() => {
// Module: crate::squeue
// Provides: {"impl_173"}
// Dependencies: {}
impl Entry128 { # [doc = " Set the submission event's [flags](Flags)."] # [inline] pub fn flags (mut self , flags : Flags) -> Entry128 { self . 0 . 0 . flags |= flags . bits () ; self } # [doc = " Clear the submission event's [flags](Flags)."] # [inline] pub fn clear_flags (mut self) -> Entry128 { self . 0 . 0 . flags = 0 ; self } # [doc = " Set the user data. This is an application-supplied value that will be passed straight"] # [doc = " through into the [completion queue entry](crate::cqueue::Entry::user_data)."] # [inline] pub fn user_data (mut self , user_data : u64) -> Entry128 { self . 0 . 0 . user_data = user_data ; self } # [doc = " Set the user data without consuming the entry."] # [inline] pub fn set_user_data (& mut self , user_data : u64) { self . 0 . 0 . user_data = user_data ; } # [doc = " Set the personality of this event. You can obtain a personality using"] # [doc = " [`Submitter::register_personality`](crate::Submitter::register_personality)."] # [inline] pub fn personality (mut self , personality : u16) -> Entry128 { self . 0 . 0 . personality = personality ; self } # [doc = " Get the opcode associated with this entry."] # [inline] pub fn get_opcode (& self) -> u32 { self . 0 . 0 . opcode . into () } }
};
}
