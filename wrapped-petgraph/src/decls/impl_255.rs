macro_rules! deps {
    () => {
        Cycle!();
        AcyclicEdgeError!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl < N > From < Cycle < N > > for AcyclicEdgeError < N > { fn from (cycle : Cycle < N >) -> Self { AcyclicEdgeError :: Cycle (cycle) } }
    };
}

impl_255!()