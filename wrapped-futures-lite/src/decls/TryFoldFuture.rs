macro_rules! TryFoldFuture {
    () => {
        # [doc = " Future for the [`StreamExt::try_fold()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryFoldFuture < 'a , S , F , B > { stream : & 'a mut S , f : F , acc : Option < B > , }
    };
}

TryFoldFuture!();