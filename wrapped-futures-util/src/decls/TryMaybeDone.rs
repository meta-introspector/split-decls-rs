macro_rules! TryMaybeDone {
    () => {
        # [doc = " A future that may have completed with an error."] # [doc = ""] # [doc = " This is created by the [`try_maybe_done()`] function."] # [derive (Debug)] pub enum TryMaybeDone < Fut : TryFuture > { # [doc = " A not-yet-completed future"] Future (Fut) , # [doc = " The output of the completed future"] Done (Fut :: Ok) , # [doc = " The empty variant after the result of a [`TryMaybeDone`] has been"] # [doc = " taken using the [`take_output`](TryMaybeDone::take_output) method,"] # [doc = " or if the future returned an error."] Gone , }
    };
}

TryMaybeDone!();