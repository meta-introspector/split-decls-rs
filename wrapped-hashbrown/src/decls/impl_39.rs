macro_rules! deps {
    () => {
        ProbeSeq!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl ProbeSeq { # [inline] fn move_next (& mut self , bucket_mask : usize) { debug_assert ! (self . stride <= bucket_mask , "Went past end of probe sequence") ; self . stride += Group :: WIDTH ; self . pos += self . stride ; self . pos &= bucket_mask ; } }
    };
}

impl_39!();