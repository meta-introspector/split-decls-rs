macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! inclusive_start {
    () => {
        deps!();
        # [inline] fn inclusive_start < T : Idx > (range : impl RangeBounds < T >) -> u32 { match range . start_bound () { Bound :: Included (start) => start . index () as u32 , Bound :: Excluded (start) => start . index () as u32 + 1 , Bound :: Unbounded => 0 , } }
    };
}

inclusive_start!()