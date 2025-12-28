macro_rules! macro_457 {
    () => {
        pin_project ! { # [doc = " Stream for the [`then`](super::StreamExt::then) method."] # [must_use = "streams do nothing unless polled"] pub struct Then < St , Fut , F > { # [pin] stream : St , # [pin] future : Option < Fut >, f : F , } }
    };
}

macro_457!();