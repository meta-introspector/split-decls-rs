macro_rules! deps {
    () => {
        RequestId!();
    };
}

macro_rules! Outgoing {
    () => {
        deps!();
        # [derive (Debug)] pub struct Outgoing < O > { next_id : i32 , pending : HashMap < RequestId , O > , }
    };
}

Outgoing!();