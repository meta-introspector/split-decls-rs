macro_rules! TryRecvError {
    () => {
        # [doc = " An error returned from the [`try_recv`] method."] # [doc = ""] # [doc = " [`try_recv`]: super::Receiver::try_recv"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub enum TryRecvError { # [doc = " A message could not be received because the channel is empty."] # [doc = ""] # [doc = " If this is a zero-capacity channel, then the error indicates that there was no sender"] # [doc = " available to send a message at the time."] Empty , # [doc = " The message could not be received because the channel is empty and disconnected."] Disconnected , }
    };
}

TryRecvError!();