macro_rules! deps {
    () => {
        ArenaChunk!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        unsafe impl < # [may_dangle] T > Drop for ArenaChunk < T > { fn drop (& mut self) { unsafe { drop (Box :: from_raw (self . storage . as_mut ())) } } }
    };
}

impl_2!()