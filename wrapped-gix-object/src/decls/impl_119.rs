macro_rules! deps {
    () => {
        EntryMode!();
        Error!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a [u8] > for tree :: EntryMode { type Error = & 'a [u8] ; fn try_from (mode : & 'a [u8]) -> Result < Self , Self :: Error > { tree :: EntryMode :: from_bytes (mode) . ok_or (mode) } }
    };
}

impl_119!()