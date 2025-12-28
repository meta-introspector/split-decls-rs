macro_rules! deps {
    () => {
        Binding!();
        Delta!();
        DiffDelta!();
        DiffFile!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl < 'a > DiffDelta < 'a > { # [doc = " Returns the flags on the delta."] # [doc = ""] # [doc = " For more information, see `DiffFlags`'s documentation."] pub fn flags (& self) -> DiffFlags { let flags = unsafe { (* self . raw) . flags } ; let mut result = DiffFlags :: empty () ; # [cfg (target_env = "msvc")] fn as_u32 (flag : i32) -> u32 { flag as u32 } # [cfg (not (target_env = "msvc"))] fn as_u32 (flag : u32) -> u32 { flag } if (flags & as_u32 (raw :: GIT_DIFF_FLAG_BINARY)) != 0 { result |= DiffFlags :: BINARY ; } if (flags & as_u32 (raw :: GIT_DIFF_FLAG_NOT_BINARY)) != 0 { result |= DiffFlags :: NOT_BINARY ; } if (flags & as_u32 (raw :: GIT_DIFF_FLAG_VALID_ID)) != 0 { result |= DiffFlags :: VALID_ID ; } if (flags & as_u32 (raw :: GIT_DIFF_FLAG_EXISTS)) != 0 { result |= DiffFlags :: EXISTS ; } result } # [doc = " Returns the number of files in this delta."] pub fn nfiles (& self) -> u16 { unsafe { (* self . raw) . nfiles } } # [doc = " Returns the status of this entry"] # [doc = ""] # [doc = " For more information, see `Delta`'s documentation"] pub fn status (& self) -> Delta { match unsafe { (* self . raw) . status } { raw :: GIT_DELTA_UNMODIFIED => Delta :: Unmodified , raw :: GIT_DELTA_ADDED => Delta :: Added , raw :: GIT_DELTA_DELETED => Delta :: Deleted , raw :: GIT_DELTA_MODIFIED => Delta :: Modified , raw :: GIT_DELTA_RENAMED => Delta :: Renamed , raw :: GIT_DELTA_COPIED => Delta :: Copied , raw :: GIT_DELTA_IGNORED => Delta :: Ignored , raw :: GIT_DELTA_UNTRACKED => Delta :: Untracked , raw :: GIT_DELTA_TYPECHANGE => Delta :: Typechange , raw :: GIT_DELTA_UNREADABLE => Delta :: Unreadable , raw :: GIT_DELTA_CONFLICTED => Delta :: Conflicted , n => panic ! ("unknown diff status: {}" , n) , } } # [doc = " Return the file which represents the \"from\" side of the diff."] # [doc = ""] # [doc = " What side this means depends on the function that was used to generate"] # [doc = " the diff and will be documented on the function itself."] pub fn old_file (& self) -> DiffFile < 'a > { unsafe { Binding :: from_raw (& (* self . raw) . old_file as * const _) } } # [doc = " Return the file which represents the \"to\" side of the diff."] # [doc = ""] # [doc = " What side this means depends on the function that was used to generate"] # [doc = " the diff and will be documented on the function itself."] pub fn new_file (& self) -> DiffFile < 'a > { unsafe { Binding :: from_raw (& (* self . raw) . new_file as * const _) } } }
    };
}

impl_330!()