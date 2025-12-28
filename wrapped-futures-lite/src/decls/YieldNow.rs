macro_rules! YieldNow {
    () => {
        # [doc = " Future for the [`yield_now()`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct YieldNow (bool) ;
    };
}

YieldNow!()