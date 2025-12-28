macro_rules! RecvError {
    () => {
        # [doc = " Error returned by the future returned by [`Receiver::recv()`] or [`UnboundedReceiver::recv()`]."] # [doc = " Received when the channel is empty and closed."] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct RecvError ;
    };
}

RecvError!()