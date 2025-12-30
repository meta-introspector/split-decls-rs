// Generated macro for FSCTL_MAILSLOT_PEEK (const)
macro_rules! Depcrate_ntioapiFSCTL_MAILSLOT_PEEK {
() => {
// Module: crate::ntioapi
// Provides: {"FSCTL_MAILSLOT_PEEK"}
// Dependencies: {}
pub const FSCTL_MAILSLOT_PEEK : u32 = CTL_CODE (FILE_DEVICE_MAILSLOT , 0 , METHOD_NEITHER , FILE_READ_DATA) ;
};
}
