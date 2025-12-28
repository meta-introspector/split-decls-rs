macro_rules! deps {
    () => {
        Feed!();
    };
}

macro_rules! Send {
    () => {
        deps!();
        # [doc = " Future for the [`send`](super::SinkExt::send) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Send < 'a , Si : ? Sized , Item > { feed : Feed < 'a , Si , Item > , }
    };
}

Send!()