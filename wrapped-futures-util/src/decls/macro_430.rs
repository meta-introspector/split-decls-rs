macro_rules! macro_430 {
    () => {
        pin_project ! { # [doc = " Stream for the [`skip_while`](super::StreamExt::skip_while) method."] # [must_use = "streams do nothing unless polled"] pub struct SkipWhile < St , Fut , F > where St : Stream { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Item >, done_skipping : bool , } }
    };
}

macro_430!();