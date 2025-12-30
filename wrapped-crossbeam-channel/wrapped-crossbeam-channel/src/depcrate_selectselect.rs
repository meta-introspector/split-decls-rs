// Generated macro for select (function)
macro_rules! Depcrate_selectselect {
() => {
// Module: crate::select
// Provides: {"select"}
// Dependencies: {}
# [doc = " Blocks until one of the operations becomes ready and selects it."] # [inline] pub fn select < 'a > (handles : & mut [(& 'a dyn SelectHandle , usize , * const u8)] , is_biased : bool ,) -> SelectedOperation < 'a > { if handles . is_empty () { panic ! ("no operations have been added to `Select`") ; } let (token , index , ptr) = run_select (handles , Timeout :: Never , is_biased) . unwrap () ; SelectedOperation { token , index , ptr , _marker : PhantomData , } }
};
}
