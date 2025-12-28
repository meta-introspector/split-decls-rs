macro_rules! macro_486 {
    () => {
        pin_project ! { # [doc = " Stream for the [`ready_chunks`](super::StreamExt::ready_chunks) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct ReadyChunks < St : Stream > { # [pin] stream : Fuse < St >, cap : usize , } }
    };
}

macro_486!();