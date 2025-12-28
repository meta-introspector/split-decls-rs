macro_rules! deps {
    () => {
        Incoming!();
        Outgoing!();
        ReqQueue!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < I , O > Default for ReqQueue < I , O > { fn default () -> ReqQueue < I , O > { ReqQueue { incoming : Incoming { pending : HashMap :: default () } , outgoing : Outgoing { next_id : 0 , pending : HashMap :: default () } , } } }
    };
}

impl_35!()