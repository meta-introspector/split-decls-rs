macro_rules! deps {
    () => {
        Storage!();
        QueueInner!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        unsafe impl < T , S : Storage > Sync for QueueInner < T , S > where T : Send { }
    };
}

impl_407!()