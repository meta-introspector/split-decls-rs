macro_rules! macro_679 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_ready_chunks`](super::TryStreamExt::try_ready_chunks) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryReadyChunks < St : TryStream > { # [pin] stream : Fuse < IntoStream < St >>, cap : usize , } }
    };
}

macro_679!();