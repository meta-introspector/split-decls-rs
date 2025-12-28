macro_rules! AlwaysReady {
    () => {
        # [doc = " Future for the [`always_ready`](always_ready()) function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct AlwaysReady < T , F : Fn () -> T > (F) ;
    };
}

AlwaysReady!()