macro_rules! deps {
    () => {
        Allocation!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Drop for Allocation { # [track_caller] fn drop (& mut self) { let location = location ! () ; rt :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; trace ! (state = ? self . state , drop . location = % location , "Allocation::drop") ; state . is_dropped = true ; }) ; } }
    };
}

impl_20!();