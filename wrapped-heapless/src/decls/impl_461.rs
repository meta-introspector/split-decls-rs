macro_rules! deps {
    () => {
        QueueInner!();
        Storage!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl < T , S : Storage > Eq for QueueInner < T , S > where T : Eq { }
    };
}

impl_461!();