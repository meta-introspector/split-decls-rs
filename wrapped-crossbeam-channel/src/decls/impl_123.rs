macro_rules! deps {
    () => {
        Slot!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < T > Slot < T > { # [doc = " Waits until a message is written into the slot."] fn wait_write (& self) { let backoff = Backoff :: new () ; while self . state . load (Ordering :: Acquire) & WRITE == 0 { backoff . snooze () ; } } }
    };
}

impl_123!();