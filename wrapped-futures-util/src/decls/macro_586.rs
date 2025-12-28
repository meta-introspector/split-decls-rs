macro_rules! macro_586 {
    () => {
        pin_project ! { # [doc = " Stream for the [`and_then`](super::TryStreamExt::and_then) method."] # [must_use = "streams do nothing unless polled"] pub struct AndThen < St , Fut , F > { # [pin] stream : St , # [pin] future : Option < Fut >, f : F , } }
    };
}

macro_586!();