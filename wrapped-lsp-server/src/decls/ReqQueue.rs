macro_rules! deps {
    () => {
        Outgoing!();
        Incoming!();
    };
}

macro_rules! ReqQueue {
    () => {
        deps!();
        # [doc = " Manages the set of pending requests, both incoming and outgoing."] # [derive (Debug)] pub struct ReqQueue < I , O > { pub incoming : Incoming < I > , pub outgoing : Outgoing < O > , }
    };
}

ReqQueue!()