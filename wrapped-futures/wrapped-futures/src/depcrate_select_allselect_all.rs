// Generated macro for select_all (function)
macro_rules! Depcrate_select_allselect_all {
() => {
// Module: crate::select_all
// Provides: {"select_all"}
// Dependencies: {}
# [doc = " Creates a new future which will select over a list of futures."] # [doc = ""] # [doc = " The returned future will wait for any future within `list` to be ready. Upon"] # [doc = " completion or failure the item resolved will be returned, along with the"] # [doc = " index of the future that was ready and the list of all the remaining"] # [doc = " futures."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the iterator specified contains no items."] pub fn select_all < I > (iter : I) -> SelectAll < < I :: Item as IntoFuture > :: Future > where I : IntoIterator , I :: Item : IntoFuture , { let ret = SelectAll { inner : iter . into_iter () . map (| a | a . into_future ()) . map (Collapsed :: Start) . map (| a | SelectAllNext { inner : a }) . collect () , } ; assert ! (ret . inner . len () > 0) ; return ret }
};
}
