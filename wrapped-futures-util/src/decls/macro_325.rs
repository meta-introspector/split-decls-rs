macro_rules! macro_325 {
    () => {
        pin_project ! { # [doc = " Stream for the [`filter_map`](super::StreamExt::filter_map) method."] # [must_use = "streams do nothing unless polled"] pub struct FilterMap < St , Fut , F > { # [pin] stream : St , f : F , # [pin] pending : Option < Fut >, } }
    };
}

macro_325!();