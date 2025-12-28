macro_rules! deps {
    () => {
        WakerToHandle!();
        NotifyWaker!();
    };
}

macro_rules! impl_1026 {
    () => {
        deps!();
        unsafe impl UnsafeNotify01 for NotifyWaker { unsafe fn clone_raw (& self) -> NotifyHandle01 { WakerToHandle (& self . 0) . into () } unsafe fn drop_raw (& self) { let ptr : * const dyn UnsafeNotify01 = self ; drop (unsafe { Box :: from_raw (ptr as * mut dyn UnsafeNotify01) }) ; } }
    };
}

impl_1026!();