macro_rules! deps {
    () => {
        RngListIter!();
        Reader!();
        Range!();
    };
}

macro_rules! RangeIterInner {
    () => {
        deps!();
        # [derive (Debug)] enum RangeIterInner < R : Reader > { Single (Option < Range >) , List (RngListIter < R >) , }
    };
}

RangeIterInner!()