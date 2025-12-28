macro_rules! deps {
    () => {
        RequestId!();
    };
}

macro_rules! Incoming {
    () => {
        deps!();
        # [derive (Debug)] pub struct Incoming < I > { pending : HashMap < RequestId , I > , }
    };
}

Incoming!();