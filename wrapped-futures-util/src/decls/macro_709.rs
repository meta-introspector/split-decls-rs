macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! macro_709 {
    () => {
        deps!();
        pin_project ! { # [doc = " Stream for the"] # [doc = " [`try_buffer_unordered`](super::TryStreamExt::try_buffer_unordered) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryBufferUnordered < St > where St : TryStream { # [pin] stream : Fuse < IntoStream < St >>, in_progress_queue : FuturesUnordered < IntoFuture < St :: Ok >>, max : Option < NonZeroUsize >, } }
    };
}

macro_709!()