macro_rules! Select {
    () => {
        # [doc = " Future for the [`select()`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] # [derive (Debug)] pub struct Select < A , B > { inner : Option < (A , B) > , }
    };
}

Select!()