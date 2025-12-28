macro_rules! deps {
    () => {
        TDEFLFlush!();
        MZFlush!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl From < MZFlush > for TDEFLFlush { fn from (flush : MZFlush) -> Self { match flush { MZFlush :: None => TDEFLFlush :: None , MZFlush :: Partial => TDEFLFlush :: Partial , MZFlush :: Sync => TDEFLFlush :: Sync , MZFlush :: Full => TDEFLFlush :: Full , MZFlush :: Finish => TDEFLFlush :: Finish , _ => TDEFLFlush :: None , } } }
    };
}

impl_33!();