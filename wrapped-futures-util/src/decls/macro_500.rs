macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! macro_500 {
    () => {
        deps!();
        pin_project ! { # [doc = " Stream for the [`buffer_unordered`](super::StreamExt::buffer_unordered)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct BufferUnordered < St > where St : Stream , { # [pin] stream : Fuse < St >, in_progress_queue : FuturesUnordered < St :: Item >, max : Option < NonZeroUsize >, } }
    };
}

macro_500!();