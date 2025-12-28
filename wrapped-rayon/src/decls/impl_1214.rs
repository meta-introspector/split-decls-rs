macro_rules! deps {
    () => {
        MergeHole!();
    };
}

macro_rules! impl_1214 {
    () => {
        deps!();
        impl < T > Drop for MergeHole < T > { fn drop (& mut self) { unsafe { let len = self . end . offset_from (self . start) as usize ; ptr :: copy_nonoverlapping (self . start , self . dest , len) ; } } }
    };
}

impl_1214!();