macro_rules! deps {
    () => {
        IoUring!();
        EntryMarker!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        # [cfg (feature = "io_safety")] impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > AsFd for IoUring < S , C > { fn as_fd (& self) -> BorrowedFd < '_ > { self . fd . as_fd () } }
    };
}

impl_216!();