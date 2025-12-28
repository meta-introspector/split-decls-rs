macro_rules! RecvError {
    () => {
        # [doc = " An error returned from the [`recv`] method."] # [doc = ""] # [doc = " A message could not be received because the channel is empty and disconnected."] # [doc = ""] # [doc = " [`recv`]: super::Receiver::recv"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct RecvError ;
    };
}

RecvError!()