// Generated macro for yield_async (function)
macro_rules! Depcrate_rt_async_supportyield_async {
() => {
// Module: crate::rt::async_support
// Provides: {"yield_async"}
// Dependencies: {}
# [doc = " The asynchronous counterpart to [`yield_blocking`]."] # [doc = ""] # [doc = " This function does not block the current task but instead gives the"] # [doc = " Rust-level executor a chance to yield control back to the host temporarily."] # [doc = " This means that other Rust-level tasks may also be able to progress during"] # [doc = " this yield operation."] # [doc = ""] # [doc = " # Return Value"] # [doc = ""] # [doc = " Unlike [`yield_blocking`] this function does not return anything. If this"] # [doc = " component task is cancelled while paused at this yield point then the future"] # [doc = " will be dropped and a Rust-level destructor will take over and clean up the"] # [doc = " task. It's not necessary to do anything with the return value of this"] # [doc = " function other than ensuring that you `.await` the function call."] pub async fn yield_async () { # [derive (Default)] struct Yield { yielded : bool , } impl Future for Yield { type Output = () ; fn poll (mut self : Pin < & mut Self > , context : & mut Context < '_ >) -> Poll < () > { if self . yielded { Poll :: Ready (()) } else { self . yielded = true ; context . waker () . wake_by_ref () ; Poll :: Pending } } } Yield :: default () . await ; }
};
}
