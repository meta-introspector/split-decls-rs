macro_rules! deps {
    () => {
        Waiter!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl Drop for Waiter { fn drop (& mut self) { unsafe { WaitForSingleObject (self . 0 , 0xFFFFFFFF) ; CloseHandle (self . 0) ; } } }
    };
}

impl_126!()