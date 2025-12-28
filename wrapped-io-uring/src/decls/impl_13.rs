macro_rules! deps {
    () => {
        IoUring!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        unsafe impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > Sync for IoUring < S , C > { }
    };
}

impl_13!()