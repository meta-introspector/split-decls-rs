macro_rules! deps {
    () => {
        WinconCapture!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl WinconCapture { fn reset (& mut self) { self . ready = None ; } }
    };
}

impl_34!();