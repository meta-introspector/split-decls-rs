macro_rules! deps {
    () => {
        BacktraceFrameFmt!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Drop for BacktraceFrameFmt < '_ , '_ , '_ > { fn drop (& mut self) { self . fmt . frame_index += 1 ; } }
    };
}

impl_42!()