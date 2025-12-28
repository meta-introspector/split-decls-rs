macro_rules! deps {
    () => {
        CompactDirection!();
        Direction!();
    };
}

macro_rules! impl_883 {
    () => {
        deps!();
        impl From < CompactDirection > for Direction { fn from (d : CompactDirection) -> Self { match d { CompactDirection :: Outgoing => Outgoing , CompactDirection :: Incoming => Incoming , } } }
    };
}

impl_883!()