// Generated macro for select_ok (function)
macro_rules! Depcrate_future_select_okselect_ok {
() => {
// Module: crate::future::select_ok
// Provides: {"select_ok"}
// Dependencies: {}
# [doc = " Creates a new future which will select the first successful future over a list of futures."] # [doc = ""] # [doc = " The returned future will wait for any future within `iter` to be ready and Ok. Unlike"] # [doc = " `select_all`, this will only return the first successful completion, or the last"] # [doc = " failure. This is useful in contexts where any success is desired and failures"] # [doc = " are ignored, unless all the futures fail."] # [doc = ""] # [doc = "  This function is only available when the `std` or `alloc` feature of this"] # [doc = " library is activated, and it is activated by default."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the iterator specified contains no items."] pub fn select_ok < I > (iter : I) -> SelectOk < I :: Item > where I : IntoIterator , I :: Item : TryFuture + Unpin , { let ret = SelectOk { inner : iter . into_iter () . collect () } ; assert ! (! ret . inner . is_empty () , "iterator provided to select_ok was empty") ; assert_future :: < Result < (< I :: Item as TryFuture > :: Ok , Vec < I :: Item >) , < I :: Item as TryFuture > :: Error > , _ , > (ret) }
};
}
