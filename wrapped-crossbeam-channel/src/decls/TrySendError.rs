macro_rules! TrySendError {
    () => {
        # [doc = " An error returned from the [`try_send`] method."] # [doc = ""] # [doc = " The error contains the message being sent so it can be recovered."] # [doc = ""] # [doc = " [`try_send`]: super::Sender::try_send"] # [derive (PartialEq , Eq , Clone , Copy)] pub enum TrySendError < T > { # [doc = " The message could not be sent because the channel is full."] # [doc = ""] # [doc = " If this is a zero-capacity channel, then the error indicates that there was no receiver"] # [doc = " available to receive the message at the time."] Full (T) , # [doc = " The message could not be sent because the channel is disconnected."] Disconnected (T) , }
    };
}

TrySendError!()