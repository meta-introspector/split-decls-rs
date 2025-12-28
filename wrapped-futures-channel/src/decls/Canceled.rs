macro_rules! deps {
    () => {
        Receiver!();
        Sender!();
    };
}

macro_rules! Canceled {
    () => {
        deps!();
        # [doc = " Error returned from a [`Receiver`] when the corresponding [`Sender`] is"] # [doc = " dropped."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct Canceled ;
    };
}

Canceled!()