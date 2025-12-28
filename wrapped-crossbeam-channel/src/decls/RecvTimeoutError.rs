macro_rules! deps {
    () => {
        Timeout!();
    };
}

macro_rules! RecvTimeoutError {
    () => {
        deps!();
        # [doc = " An error returned from the [`recv_timeout`] method."] # [doc = ""] # [doc = " [`recv_timeout`]: super::Receiver::recv_timeout"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub enum RecvTimeoutError { # [doc = " A message could not be received because the channel is empty and the operation timed out."] # [doc = ""] # [doc = " If this is a zero-capacity channel, then the error indicates that there was no sender"] # [doc = " available to send a message and the operation timed out."] Timeout , # [doc = " The message could not be received because the channel is empty and disconnected."] Disconnected , }
    };
}

RecvTimeoutError!()