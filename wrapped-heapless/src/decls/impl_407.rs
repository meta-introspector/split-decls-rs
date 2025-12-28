macro_rules! deps {
    () => {
        QueueInner!();
        Storage!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        unsafe impl < T , S : Storage > Sync for QueueInner < T , S > where T : Send { }
    };
}

impl_407!();