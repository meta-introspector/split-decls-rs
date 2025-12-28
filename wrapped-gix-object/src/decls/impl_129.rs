macro_rules! deps {
    () => {
        Error!();
        EntryMode!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl TryFrom < u32 > for tree :: EntryMode { type Error = u32 ; fn try_from (mode : u32) -> Result < Self , Self :: Error > { Ok (match mode { 0o40000 | 0o120000 | 0o160000 => EntryMode { internal : mode as u16 } , blob_mode if blob_mode & 0o100000 == 0o100000 => EntryMode { internal : mode as u16 } , _ => return Err (mode) , }) } }
    };
}

impl_129!()