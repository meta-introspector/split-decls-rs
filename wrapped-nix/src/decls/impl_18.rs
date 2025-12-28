macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl TryFrom < io :: Error > for Errno { type Error = io :: Error ; fn try_from (ioerror : io :: Error) -> std :: result :: Result < Self , io :: Error > { ioerror . raw_os_error () . map (Errno :: from_raw) . ok_or (ioerror) } }
    };
}

impl_18!()