macro_rules! macro_702 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_take_while`](super::TryStreamExt::try_take_while)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct TryTakeWhile < St , Fut , F > where St : TryStream , { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Ok >, done_taking : bool , } }
    };
}

macro_702!()