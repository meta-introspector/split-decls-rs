// Generated macro for select_all (function)
macro_rules! Depcrate_future_select_allselect_all {
() => {
// Module: crate::future::select_all
// Provides: {"select_all"}
// Dependencies: {}
# [doc = " Creates a new future which will select over a list of futures."] # [doc = ""] # [doc = " The returned future will wait for any future within `iter` to be ready. Upon"] # [doc = " completion the item resolved will be returned, along with the index of the"] # [doc = " future that was ready and the list of all the remaining futures."] # [doc = ""] # [doc = " There are no guarantees provided on the order of the list with the remaining"] # [doc = " futures. They might be swapped around, reversed, or completely random."] # [doc = ""] # [doc = " This function is only available when the `std` or `alloc` feature of this"] # [doc = " library is activated, and it is activated by default."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the iterator specified contains no items."] pub fn select_all < I > (iter : I) -> SelectAll < I :: Item > where I : IntoIterator , I :: Item : Future + Unpin , { let ret = SelectAll { inner : iter . into_iter () . collect () } ; assert ! (! ret . inner . is_empty ()) ; assert_future :: < (< I :: Item as Future > :: Output , usize , Vec < I :: Item >) , _ > (ret) }
};
}
