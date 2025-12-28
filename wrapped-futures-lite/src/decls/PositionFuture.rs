macro_rules! PositionFuture {
    () => {
        # [doc = " Future for the [`StreamExt::position()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PositionFuture < 'a , S : ? Sized , P > { stream : & 'a mut S , predicate : P , index : usize , }
    };
}

PositionFuture!()