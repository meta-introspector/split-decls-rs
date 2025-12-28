macro_rules! deps {
    () => {
        FuturesOrdered!();
        MaybeDone!();
    };
}

macro_rules! JoinAllKind {
    () => {
        deps!();
        enum JoinAllKind < F > where F : Future , { Small { elems : Pin < Box < [MaybeDone < F >] > > , } , # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] Big { fut : Collect < FuturesOrdered < F > , Vec < F :: Output > > , } , }
    };
}

JoinAllKind!();