macro_rules! deps {
    () => {
        File!();
        Error!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl TryFrom < & Path > for File { type Error = Error ; fn try_from (path : & Path) -> Result < Self , Self :: Error > { let data = std :: fs :: File :: open (path) . and_then (| file | { # [allow (unsafe_code)] unsafe { memmap2 :: MmapOptions :: new () . map_copy_read_only (& file) } }) . map_err (| e | Error :: Io { err : e , path : path . to_owned () , }) ? ; Self :: new (data , path . to_owned ()) } }
    };
}

impl_30!();