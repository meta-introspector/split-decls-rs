macro_rules! Ready {
    () => {
        # [doc = " Future for the [`ready`](ready()) function."] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Ready < T > (Option < T >) ;
    };
}

Ready!();