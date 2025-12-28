macro_rules! deps {
    () => {
        IoUring!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [cfg (feature = "io_safety")] impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > AsFd for IoUring < S , C > { fn as_fd (& self) -> BorrowedFd < '_ > { self . fd . as_fd () } }
    };
}

impl_21!()