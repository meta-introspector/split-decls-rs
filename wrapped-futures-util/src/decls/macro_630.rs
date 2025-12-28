macro_rules! macro_630 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_filter_map`](super::TryStreamExt::try_filter_map)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct TryFilterMap < St , Fut , F > { # [pin] stream : St , f : F , # [pin] pending : Option < Fut >, } }
    };
}

macro_630!();