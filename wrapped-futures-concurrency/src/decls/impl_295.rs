macro_rules! deps {
    () => {
        RaceOk!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        # [pinned_drop] impl < Fut , T , E , const N : usize > PinnedDrop for RaceOk < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > , { fn drop (self : Pin < & mut Self >) { let this = self . project () ; for (st , err) in this . error_states . iter_mut () . zip (this . errors . iter_mut ()) . filter (| (st , _err) | st . is_ready ()) { unsafe { err . assume_init_drop () } ; st . set_none () ; } } }
    };
}

impl_295!()