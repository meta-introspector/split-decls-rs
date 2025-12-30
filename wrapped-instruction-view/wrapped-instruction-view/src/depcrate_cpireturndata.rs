// Generated macro for ReturnData (struct)
macro_rules! Depcrate_cpiReturnData {
() => {
// Module: crate::cpi
// Provides: {"ReturnData"}
// Dependencies: {}
# [doc = " Struct to hold the return data from an invoked program."] # [derive (Debug)] pub struct ReturnData { # [doc = " Program that most recently set the return data."] program_id : Address , # [doc = " Return data set by the program."] data : [MaybeUninit < u8 > ; MAX_RETURN_DATA] , # [doc = " Length of the return data."] size : usize , }
};
}
