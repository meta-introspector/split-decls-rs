macro_rules! deps {
    () => {
        Message!();
        WakeHandle!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl ArcWake for WakeHandle { fn wake_by_ref (arc_self : & Arc < Self >) { if let Ok (task) = arc_self . mutex . notify () { arc_self . exec . state . send (Message :: Run (task)) } } }
    };
}

impl_40!()