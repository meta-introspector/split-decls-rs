macro_rules! deps {
    () => {
        IoUring!();
        EntryMarker!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        unsafe impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > Sync for IoUring < S , C > { }
    };
}

impl_208!()