macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! Cancellation {
    () => {
        deps!();
        # [doc = " A future that resolves when the receiving end of a channel has hung up."] # [doc = ""] # [doc = " This is an `.await`-friendly interface around [`poll_canceled`](Sender::poll_canceled)."] # [must_use = "futures do nothing unless you `.await` or poll them"] # [derive (Debug)] pub struct Cancellation < 'a , T > { inner : & 'a mut Sender < T > , }
    };
}

Cancellation!();