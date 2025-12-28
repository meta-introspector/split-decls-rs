macro_rules! macro_668 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_chunks`](super::TryStreamExt::try_chunks) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryChunks < St : TryStream > { # [pin] stream : Fuse < IntoStream < St >>, items : Vec < St :: Ok >, cap : usize , } }
    };
}

macro_668!();