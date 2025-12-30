// Generated macro for try_select (function)
macro_rules! Depcrate_selecttry_select {
() => {
// Module: crate::select
// Provides: {"try_select"}
// Dependencies: {}
# [doc = " Attempts to select one of the operations without blocking."] # [inline] pub fn try_select < 'a > (handles : & mut [(& 'a dyn SelectHandle , usize , * const u8)] , is_biased : bool ,) -> Result < SelectedOperation < 'a > , TrySelectError > { match run_select (handles , Timeout :: Now , is_biased) { None => Err (TrySelectError) , Some ((token , index , ptr)) => Ok (SelectedOperation { token , index , ptr , _marker : PhantomData , }) , } }
};
}
