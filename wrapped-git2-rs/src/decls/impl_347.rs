macro_rules! deps {
    () => {
        DiffHunk!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < 'a > DiffHunk < 'a > { # [doc = " Starting line number in old_file"] pub fn old_start (& self) -> u32 { unsafe { (* self . raw) . old_start as u32 } } # [doc = " Number of lines in old_file"] pub fn old_lines (& self) -> u32 { unsafe { (* self . raw) . old_lines as u32 } } # [doc = " Starting line number in new_file"] pub fn new_start (& self) -> u32 { unsafe { (* self . raw) . new_start as u32 } } # [doc = " Number of lines in new_file"] pub fn new_lines (& self) -> u32 { unsafe { (* self . raw) . new_lines as u32 } } # [doc = " Header text"] pub fn header (& self) -> & 'a [u8] { unsafe { slice :: from_raw_parts ((* self . raw) . header . as_ptr () as * const u8 , (* self . raw) . header_len as usize ,) } } }
    };
}

impl_347!()