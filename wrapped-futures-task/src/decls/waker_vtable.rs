macro_rules! deps {
    () => {
        ArcWake!();
    };
}

macro_rules! waker_vtable {
    () => {
        deps!();
        pub (super) fn waker_vtable < W : ArcWake + 'static > () -> & 'static RawWakerVTable { & RawWakerVTable :: new (clone_arc_raw :: < W > , wake_arc_raw :: < W > , wake_by_ref_arc_raw :: < W > , drop_arc_raw :: < W > ,) }
    };
}

waker_vtable!()