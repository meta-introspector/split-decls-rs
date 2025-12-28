macro_rules! macro_464 {
    () => {
        pin_project ! { # [doc = " Future for the [`try_for_each`](super::StreamExt::try_for_each) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryForEach < St , Fut , F > { # [pin] stream : St , f : F , # [pin] future : Option < Fut >, } }
    };
}

macro_464!()