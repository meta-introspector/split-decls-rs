macro_rules! deps {
    () => {
        IoUring!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        unsafe impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > Send for IoUring < S , C > { }
    };
}

impl_12!()