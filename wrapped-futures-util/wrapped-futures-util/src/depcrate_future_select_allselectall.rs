// Generated macro for SelectAll (struct)
macro_rules! Depcrate_future_select_allSelectAll {
() => {
// Module: crate::future::select_all
// Provides: {"SelectAll"}
// Dependencies: {}
# [doc = " Future for the [`select_all`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct SelectAll < Fut > { inner : Vec < Fut > , }
};
}
