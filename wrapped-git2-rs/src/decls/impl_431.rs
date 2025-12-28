macro_rules! deps {
    () => {
        MergeFileResult!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl MergeFileResult { # [doc = " True if the output was automerged, false if the output contains"] # [doc = " conflict markers."] pub fn is_automergeable (& self) -> bool { self . raw . automergeable > 0 } # [doc = " The path that the resultant merge file should use."] # [doc = ""] # [doc = " returns `None` if a filename conflict would occur,"] # [doc = " or if the path is not valid utf-8"] pub fn path (& self) -> Option < & str > { self . path_bytes () . and_then (| bytes | str :: from_utf8 (bytes) . ok ()) } # [doc = " Gets the path as a byte slice."] pub fn path_bytes (& self) -> Option < & [u8] > { unsafe { crate :: opt_bytes (self , self . raw . path) } } # [doc = " The mode that the resultant merge file should use."] pub fn mode (& self) -> u32 { self . raw . mode as u32 } # [doc = " The contents of the merge."] pub fn content (& self) -> & [u8] { unsafe { std :: slice :: from_raw_parts (self . raw . ptr as * const u8 , self . raw . len as usize) } } }
    };
}

impl_431!();