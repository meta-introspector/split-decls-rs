macro_rules! deps {
    () => {
        TryMaybeDone!();
        FuturesOrdered!();
    };
}

macro_rules! TryJoinAllKind {
    () => {
        deps!();
        enum TryJoinAllKind < F > where F : TryFuture , { Small { elems : Pin < Box < [TryMaybeDone < IntoFuture < F > >] > > , } , # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] Big { fut : TryCollect < FuturesOrdered < IntoFuture < F > > , Vec < F :: Ok > > , } , }
    };
}

TryJoinAllKind!();