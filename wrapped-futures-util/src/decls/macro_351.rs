macro_rules! macro_351 {
    () => {
        pin_project ! { # [doc = " Future for the [`all`](super::StreamExt::all) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct All < St , Fut , F > { # [pin] stream : St , f : F , done : bool , # [pin] future : Option < Fut >, } }
    };
}

macro_351!()