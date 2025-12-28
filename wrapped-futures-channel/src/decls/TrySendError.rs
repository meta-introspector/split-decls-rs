macro_rules! deps {
    () => {
        SendError!();
    };
}

macro_rules! TrySendError {
    () => {
        deps!();
        # [doc = " The error type returned from [`try_send`](Sender::try_send)."] # [derive (Clone , PartialEq , Eq)] pub struct TrySendError < T > { err : SendError , val : T , }
    };
}

TrySendError!();