macro_rules! macro_664 {
    () => {
        pin_project ! { # [doc = " Future for the [`try_concat`](super::TryStreamExt::try_concat) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryConcat < St : TryStream > { # [pin] stream : St , accum : Option < St :: Ok >, } }
    };
}

macro_664!()