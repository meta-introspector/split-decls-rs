macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! collect_from_iter {
    () => {
        deps!();
        # [test] fn collect_from_iter () { struct IterNoHint < I : Iterator > (I) ; impl < I : Iterator > Iterator for IterNoHint < I > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } } let iter = IterNoHint (std :: iter :: repeat (1u8) . take (1_000_000)) ; let _y : SmallVec < u8 , 1 > = SmallVec :: from_iter (iter) ; }
    };
}

collect_from_iter!()