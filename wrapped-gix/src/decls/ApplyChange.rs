macro_rules! ApplyChange {
    () => {
        pub (super) enum ApplyChange { SetSizeToZero , NewStat (crate :: index :: entry :: Stat) , }
    };
}

ApplyChange!();