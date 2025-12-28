macro_rules! deps {
    () => {
        FlowController!();
        FlowStep!();
    };
}

macro_rules! impl_539 {
    () => {
        deps!();
        impl < I , O > FlowController < I , O > for () { fn next_step (item : I) -> FlowStep < I , O > { FlowStep :: Continue (item) } }
    };
}

impl_539!()