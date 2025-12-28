macro_rules! AnyFuture {
    () => {
        # [doc = " Future for the [`StreamExt::any()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct AnyFuture < 'a , S : ? Sized , P > { stream : & 'a mut S , predicate : P , }
    };
}

AnyFuture!();