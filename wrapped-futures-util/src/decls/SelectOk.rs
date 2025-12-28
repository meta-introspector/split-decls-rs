macro_rules! SelectOk {
    () => {
        # [doc = " Future for the [`select_ok`] function."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct SelectOk < Fut > { inner : Vec < Fut > , }
    };
}

SelectOk!()