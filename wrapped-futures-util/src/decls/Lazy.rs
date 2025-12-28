macro_rules! Lazy {
    () => {
        # [doc = " Future for the [`lazy`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Lazy < F > { f : Option < F > , }
    };
}

Lazy!()