// Generated macro for mounts (function)
macro_rules! Depcrate_fs_mounts_macosmounts {
() => {
// Module: crate::fs::mounts::macos
// Provides: {"mounts"}
// Dependencies: {}
# [doc = " Get a list of all mounted filesystem"] pub fn mounts () -> Result < Vec < MountedFs > , Error > { let mut count : i32 = unsafe { getfsstat (ptr :: null_mut () , 0 , MNT_NOWAIT) } ; let mut mntbuf = Vec :: < statfs > :: new () ; if count > 0 { mntbuf . resize_with (count as usize , | | unsafe { mem :: zeroed () }) ; let bufsize = mntbuf . len () * mem :: size_of :: < statfs > () ; count = unsafe { getfsstat (mntbuf . as_mut_ptr () , bufsize as c_int , MNT_NOWAIT) } ; if count >= 0 { mntbuf . truncate (count as usize) ; } } if count < 0 { return Err (Error :: GetFSStatError (unsafe { * __error () })) ; } let mut mounts = Vec :: with_capacity (count as usize) ; for mnt in & mntbuf { let mount_point = OsStr :: from_bytes (unsafe { CStr :: from_ptr (mnt . f_mntonname . as_ptr () . cast :: < c_char > ()) } . to_bytes () ,) ; let dest = PathBuf :: from (mount_point) ; let fstype = unsafe { CStr :: from_ptr (mnt . f_fstypename . as_ptr () . cast :: < c_char > ()) } . to_string_lossy () . into () ; let source = unsafe { CStr :: from_ptr (mnt . f_mntfromname . as_ptr () . cast :: < c_char > ()) } . to_string_lossy () . into () ; mounts . push (MountedFs { dest , fstype , source , }) ; } Ok (mounts) }
};
}
