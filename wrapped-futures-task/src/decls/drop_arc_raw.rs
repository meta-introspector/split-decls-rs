macro_rules! deps {
    () => {
        ArcWake!();
    };
}

macro_rules! drop_arc_raw {
    () => {
        deps!();
        unsafe fn drop_arc_raw < T : ArcWake + 'static > (data : * const ()) { drop (unsafe { Arc :: < T > :: from_raw (data . cast :: < T > ()) }) }
    };
}

drop_arc_raw!()