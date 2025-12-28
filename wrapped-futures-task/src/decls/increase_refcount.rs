macro_rules! deps {
    () => {
        ArcWake!();
    };
}

macro_rules! increase_refcount {
    () => {
        deps!();
        unsafe fn increase_refcount < T : ArcWake + 'static > (data : * const ()) { let arc = mem :: ManuallyDrop :: new (unsafe { Arc :: < T > :: from_raw (data . cast :: < T > ()) }) ; let _arc_clone : mem :: ManuallyDrop < _ > = arc . clone () ; }
    };
}

increase_refcount!()