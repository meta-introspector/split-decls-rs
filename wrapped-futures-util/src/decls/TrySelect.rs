macro_rules! TrySelect {
    () => {
        # [doc = " Future for the [`try_select()`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] # [derive (Debug)] pub struct TrySelect < A , B > { inner : Option < (A , B) > , }
    };
}

TrySelect!();