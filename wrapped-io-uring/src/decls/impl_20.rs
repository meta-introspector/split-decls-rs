macro_rules! deps {
    () => {
        IoUring!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > AsRawFd for IoUring < S , C > { fn as_raw_fd (& self) -> RawFd { self . fd . as_raw_fd () } }
    };
}

impl_20!()