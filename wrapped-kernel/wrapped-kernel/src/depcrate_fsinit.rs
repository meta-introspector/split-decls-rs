// Generated macro for init (function)
macro_rules! Depcrate_fsinit {
() => {
// Module: crate::fs
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init () { const VERSION : & str = env ! ("CARGO_PKG_VERSION") ; const UTC_BUILT_TIME : & str = build_time :: build_time_utc ! () ; FILESYSTEM . set (Filesystem :: new ()) . unwrap () ; FILESYSTEM . get () . unwrap () . mkdir ("/tmp" , AccessPermission :: from_bits (0o777) . unwrap ()) . expect ("Unable to create /tmp") ; FILESYSTEM . get () . unwrap () . mkdir ("/proc" , AccessPermission :: from_bits (0o777) . unwrap ()) . expect ("Unable to create /proc") ; if let Ok (mut file) = File :: create ("/proc/version") { if write ! (file , "HermitOS version {VERSION} # UTC {UTC_BUILT_TIME}") . is_err () { error ! ("Unable to write in /proc/version") ; } } else { error ! ("Unable to create /proc/version") ; } let mut cwd = WORKING_DIRECTORY . lock () ; * cwd = Some ("/tmp" . to_string ()) ; drop (cwd) ; # [cfg (all (feature = "fuse" , feature = "pci"))] fuse :: init () ; if crate :: env :: is_uhyve () { uhyve :: init () ; } }
};
}
