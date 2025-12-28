macro_rules! macro_318 {
    () => {
        pin_project ! { # [doc = " Stream for the [`filter`](super::StreamExt::filter) method."] # [must_use = "streams do nothing unless polled"] pub struct Filter < St , Fut , F > where St : Stream , { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Item >, } }
    };
}

macro_318!();