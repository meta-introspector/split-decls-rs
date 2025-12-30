// Generated macro for impl_1329 (impl)
macro_rules! Depcrate_optionimpl_1329 {
() => {
// Module: crate::option
// Provides: {"impl_1329"}
// Dependencies: {}
# [doc = " Collect an arbitrary `Option`-wrapped collection."] # [doc = ""] # [doc = " If any item is `None`, then all previous items collected are discarded,"] # [doc = " and it returns only `None`."] impl < C , T > FromParallelIterator < Option < T > > for Option < C > where C : FromParallelIterator < T > , T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = Option < T > > , { fn check < T > (found_none : & AtomicBool) -> impl Fn (& Option < T >) + '_ { move | item | { if item . is_none () { found_none . store (true , Ordering :: Relaxed) ; } } } let found_none = AtomicBool :: new (false) ; let collection = par_iter . into_par_iter () . inspect (check (& found_none)) . while_some () . collect () ; if found_none . load (Ordering :: Relaxed) { None } else { Some (collection) } } }
};
}
