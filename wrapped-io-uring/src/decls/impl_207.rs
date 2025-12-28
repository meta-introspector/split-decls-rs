macro_rules! deps {
    () => {
        IoUring!();
        EntryMarker!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        unsafe impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > Send for IoUring < S , C > { }
    };
}

impl_207!();