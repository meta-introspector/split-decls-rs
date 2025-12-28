macro_rules! Recv {
    () => {
        # [doc = " Future returned by [`Receiver::recv()`] or [`UnboundedReceiver::recv()`]."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Recv < 'a , St : ? Sized > { stream : & 'a mut St , }
    };
}

Recv!();