macro_rules! AllFuture {
    () => {
        # [doc = " Future for the [`StreamExt::all()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct AllFuture < 'a , S : ? Sized , P > { stream : & 'a mut S , predicate : P , }
    };
}

AllFuture!();