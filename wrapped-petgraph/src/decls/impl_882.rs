macro_rules! deps {
    () => {
        CompactDirection!();
        Direction!();
    };
}

macro_rules! impl_882 {
    () => {
        deps!();
        impl From < Direction > for CompactDirection { fn from (d : Direction) -> Self { match d { Outgoing => CompactDirection :: Outgoing , Incoming => CompactDirection :: Incoming , } } }
    };
}

impl_882!();