macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! macro_507 {
    () => {
        deps!();
        pin_project ! { # [doc = " Stream for the [`buffered`](super::StreamExt::buffered) method."] # [must_use = "streams do nothing unless polled"] pub struct Buffered < St > where St : Stream , St :: Item : Future , { # [pin] stream : Fuse < St >, in_progress_queue : FuturesOrdered < St :: Item >, max : Option < NonZeroUsize >, } }
    };
}

macro_507!();