macro_rules! macro_363 {
    () => {
        pin_project ! { # [doc = " Future for the [`for_each`](super::StreamExt::for_each) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ForEach < St , Fut , F > { # [pin] stream : St , f : F , # [pin] future : Option < Fut >, } }
    };
}

macro_363!();