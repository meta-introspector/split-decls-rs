macro_rules! macro_695 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_skip_while`](super::TryStreamExt::try_skip_while)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct TrySkipWhile < St , Fut , F > where St : TryStream { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Ok >, done_skipping : bool , } }
    };
}

macro_695!();