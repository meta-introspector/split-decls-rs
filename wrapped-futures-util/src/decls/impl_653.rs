macro_rules! deps {
    () => {
        BaseStreamItem!();
        Either!();
        PropagateBaseStreamError!();
        FlowController!();
        FlowStep!();
        InnerStreamItem!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        impl < St > FlowController < BaseStreamItem < St > , InnerStreamItem < St > > for PropagateBaseStreamError < St > where St : TryStream , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < St :: Error > , { fn next_step (item : BaseStreamItem < St >) -> FlowStep < BaseStreamItem < St > , InnerStreamItem < St > > { match item { st @ Either :: Left (_) => FlowStep :: Continue (st) , Either :: Right (mut err) => FlowStep :: Return (err . next_immediate () . unwrap ()) , } } }
    };
}

impl_653!()