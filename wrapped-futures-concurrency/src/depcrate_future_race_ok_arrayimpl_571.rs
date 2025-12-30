// Generated macro for impl_571 (impl)
macro_rules! Depcrate_future_race_ok_arrayimpl_571 {
() => {
// Module: crate::future::race_ok::array
// Provides: {"impl_571"}
// Dependencies: {}
# [pinned_drop] impl < Fut , T , E , const N : usize > PinnedDrop for RaceOk < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > , { fn drop (self : Pin < & mut Self >) { let this = self . project () ; for (st , err) in this . error_states . iter_mut () . zip (this . errors . iter_mut ()) . filter (| (st , _err) | st . is_ready ()) { unsafe { err . assume_init_drop () } ; st . set_none () ; } } }
};
}
