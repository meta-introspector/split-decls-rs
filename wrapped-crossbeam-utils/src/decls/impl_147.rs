macro_rules! deps {
    () => {
        WaitGroup!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Drop for WaitGroup { fn drop (& mut self) { let mut count = self . inner . count . lock () . unwrap () ; * count -= 1 ; if * count == 0 { self . inner . cvar . notify_all () ; } } }
    };
}

impl_147!();