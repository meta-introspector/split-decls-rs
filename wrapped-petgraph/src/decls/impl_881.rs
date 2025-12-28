macro_rules! deps {
    () => {
        CompactDirection!();
    };
}

macro_rules! impl_881 {
    () => {
        deps!();
        impl CompactDirection { # [doc = " Return the opposite `CompactDirection`."] # [inline] pub fn opposite (self) -> CompactDirection { match self { CompactDirection :: Outgoing => CompactDirection :: Incoming , CompactDirection :: Incoming => CompactDirection :: Outgoing , } } }
    };
}

impl_881!();