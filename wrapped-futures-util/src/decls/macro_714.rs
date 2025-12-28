macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! macro_714 {
    () => {
        deps!();
        pin_project ! { # [doc = " Stream for the [`try_buffered`](super::TryStreamExt::try_buffered) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryBuffered < St > where St : TryStream , St :: Ok : TryFuture , { # [pin] stream : Fuse < IntoStream < St >>, in_progress_queue : FuturesOrdered < IntoFuture < St :: Ok >>, max : Option < NonZeroUsize >, } }
    };
}

macro_714!();