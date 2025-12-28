macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl Drop for RootDatabase { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . storage) } ; } }
    };
}

impl_249!()