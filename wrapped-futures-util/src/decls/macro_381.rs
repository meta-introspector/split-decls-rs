macro_rules! macro_381 {
    () => {
        pin_project ! { # [doc = " Stream for the [`map`](super::StreamExt::map) method."] # [must_use = "streams do nothing unless polled"] pub struct Map < St , F > { # [pin] stream : St , f : F , } }
    };
}

macro_381!()