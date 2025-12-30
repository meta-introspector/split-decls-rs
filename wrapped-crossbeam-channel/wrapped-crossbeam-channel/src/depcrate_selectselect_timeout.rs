// Generated macro for select_timeout (function)
macro_rules! Depcrate_selectselect_timeout {
() => {
// Module: crate::select
// Provides: {"select_timeout"}
// Dependencies: {}
# [doc = " Blocks for a limited time until one of the operations becomes ready and selects it."] # [inline] pub fn select_timeout < 'a > (handles : & mut [(& 'a dyn SelectHandle , usize , * const u8)] , timeout : Duration , is_biased : bool ,) -> Result < SelectedOperation < 'a > , SelectTimeoutError > { match Instant :: now () . checked_add (timeout) { Some (deadline) => select_deadline (handles , deadline , is_biased) , None => Ok (select (handles , is_biased)) , } }
};
}
