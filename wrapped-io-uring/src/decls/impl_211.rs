macro_rules! deps {
    () => {
        IoUring!();
        EntryMarker!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > Drop for IoUring < S , C > { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . memory) ; } } }
    };
}

impl_211!();