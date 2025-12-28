macro_rules! deps {
    () => {
        ArcWake!();
    };
}

macro_rules! wake_arc_raw {
    () => {
        deps!();
        unsafe fn wake_arc_raw < T : ArcWake + 'static > (data : * const ()) { let arc : Arc < T > = unsafe { Arc :: from_raw (data . cast :: < T > ()) } ; ArcWake :: wake (arc) ; }
    };
}

wake_arc_raw!()