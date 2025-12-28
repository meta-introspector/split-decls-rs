macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Drop for Symbol { # [inline] fn drop (& mut self) { let Some (arc) = (unsafe { self . repr . try_as_arc_owned () }) else { return ; } ; if Arc :: count (& arc) == 2 { Self :: drop_slow (self) ; } ManuallyDrop :: into_inner (arc) ; } }
    };
}

impl_21!()