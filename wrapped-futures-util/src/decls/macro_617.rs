macro_rules! macro_617 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_filter`](super::TryStreamExt::try_filter)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct TryFilter < St , Fut , F > where St : TryStream { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Ok >, } }
    };
}

macro_617!();