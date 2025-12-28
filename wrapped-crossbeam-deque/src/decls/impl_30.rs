macro_rules! deps {
    () => {
        Slot!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T > Slot < T > { # [doc = " Waits until a task is written into the slot."] fn wait_write (& self) { let backoff = Backoff :: new () ; while self . state . load (Ordering :: Acquire) & WRITE == 0 { backoff . snooze () ; } } }
    };
}

impl_30!()