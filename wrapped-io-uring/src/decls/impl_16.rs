macro_rules! deps {
    () => {
        IoUring!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > Drop for IoUring < S , C > { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . memory) ; } } }
    };
}

impl_16!()