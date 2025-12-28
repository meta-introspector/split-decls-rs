macro_rules! macro_604 {
    () => {
        pin_project ! { # [doc = " Stream for the [`or_else`](super::TryStreamExt::or_else) method."] # [must_use = "streams do nothing unless polled"] pub struct OrElse < St , Fut , F > { # [pin] stream : St , # [pin] future : Option < Fut >, f : F , } }
    };
}

macro_604!();