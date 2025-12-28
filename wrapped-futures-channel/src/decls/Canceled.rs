macro_rules! deps {
    () => {
        Sender!();
        Receiver!();
    };
}

macro_rules! Canceled {
    () => {
        deps!();
        # [doc = " Error returned from a [`Receiver`] when the corresponding [`Sender`] is"] # [doc = " dropped."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct Canceled ;
    };
}

Canceled!();