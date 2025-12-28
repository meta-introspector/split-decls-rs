macro_rules! FindFuture {
    () => {
        # [doc = " Future for the [`StreamExt::find()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FindFuture < 'a , S : ? Sized , P > { stream : & 'a mut S , predicate : P , }
    };
}

FindFuture!()