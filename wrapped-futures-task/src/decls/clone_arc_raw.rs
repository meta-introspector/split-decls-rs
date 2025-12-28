macro_rules! deps {
    () => {
        ArcWake!();
    };
}

macro_rules! clone_arc_raw {
    () => {
        deps!();
        # [inline (always)] unsafe fn clone_arc_raw < T : ArcWake + 'static > (data : * const ()) -> RawWaker { unsafe { increase_refcount :: < T > (data) } RawWaker :: new (data , waker_vtable :: < T > ()) }
    };
}

clone_arc_raw!()