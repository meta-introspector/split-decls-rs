// Generated macro for block_on (function)
macro_rules! Depcrate_parallel_async_executorblock_on {
() => {
// Module: crate::parallel::async_executor
// Provides: {"block_on"}
// Dependencies: {}
# [doc = " Execute the futures and return when they are all done."] # [doc = ""] # [doc = " Here we use our own homebrew async executor since cc is used in the build"] # [doc = " script of many popular projects, pulling in additional dependencies would"] # [doc = " significantly slow down its compilation."] pub (crate) fn block_on < Fut1 , Fut2 > (mut fut1 : Fut1 , mut fut2 : Fut2 , has_made_progress : & Cell < bool > ,) -> Result < () , Error > where Fut1 : Future < Output = Result < () , Error > > , Fut2 : Future < Output = Result < () , Error > > , { let mut fut1 = Some (unsafe { Pin :: new_unchecked (& mut fut1) }) ; let mut fut2 = Some (unsafe { Pin :: new_unchecked (& mut fut2) }) ; let waker = unsafe { Waker :: from_raw (NOOP_RAW_WAKER) } ; let mut context = Context :: from_waker (& waker) ; let mut backoff_cnt = 0 ; loop { has_made_progress . set (false) ; if let Some (fut) = fut2 . as_mut () { if let Poll :: Ready (res) = fut . as_mut () . poll (& mut context) { fut2 = None ; res ? ; } } if let Some (fut) = fut1 . as_mut () { if let Poll :: Ready (res) = fut . as_mut () . poll (& mut context) { fut1 = None ; res ? ; } } if fut1 . is_none () && fut2 . is_none () { return Ok (()) ; } if ! has_made_progress . get () { if backoff_cnt > 3 { let duration = Duration :: from_millis (100 * (backoff_cnt - 3) . min (10)) ; thread :: sleep (duration) ; } else { thread :: yield_now () ; } } backoff_cnt = if has_made_progress . get () { 0 } else { backoff_cnt + 1 } ; } }
};
}
