macro_rules! deps {
    () => {
        StatusEntry!();
        Binding!();
        DiffDelta!();
    };
}

macro_rules! impl_759 {
    () => {
        deps!();
        impl < 'statuses > StatusEntry < 'statuses > { # [doc = " Access the bytes for this entry's corresponding pathname"] pub fn path_bytes (& self) -> & [u8] { unsafe { if (* self . raw) . head_to_index . is_null () { crate :: opt_bytes (self , (* (* self . raw) . index_to_workdir) . old_file . path) } else { crate :: opt_bytes (self , (* (* self . raw) . head_to_index) . old_file . path) } . unwrap () } } # [doc = " Access this entry's path name as a string."] # [doc = ""] # [doc = " Returns `None` if the path is not valid utf-8."] pub fn path (& self) -> Option < & str > { str :: from_utf8 (self . path_bytes ()) . ok () } # [doc = " Access the status flags for this file"] pub fn status (& self) -> Status { Status :: from_bits_truncate (unsafe { (* self . raw) . status as u32 }) } # [doc = " Access detailed information about the differences between the file in"] # [doc = " HEAD and the file in the index."] pub fn head_to_index (& self) -> Option < DiffDelta < 'statuses > > { unsafe { Binding :: from_raw_opt ((* self . raw) . head_to_index) } } # [doc = " Access detailed information about the differences between the file in"] # [doc = " the index and the file in the working directory."] pub fn index_to_workdir (& self) -> Option < DiffDelta < 'statuses > > { unsafe { Binding :: from_raw_opt ((* self . raw) . index_to_workdir) } } }
    };
}

impl_759!()