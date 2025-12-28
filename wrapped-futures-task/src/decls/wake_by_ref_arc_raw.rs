macro_rules! deps {
    () => {
        ArcWake!();
    };
}

macro_rules! wake_by_ref_arc_raw {
    () => {
        deps!();
        unsafe fn wake_by_ref_arc_raw < T : ArcWake + 'static > (data : * const ()) { let arc = mem :: ManuallyDrop :: new (unsafe { Arc :: < T > :: from_raw (data . cast :: < T > ()) }) ; ArcWake :: wake_by_ref (& arc) ; }
    };
}

wake_by_ref_arc_raw!()