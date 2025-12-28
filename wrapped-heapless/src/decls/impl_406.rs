macro_rules! deps {
    () => {
        QueueInner!();
        Storage!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl < T , S : Storage > Drop for QueueInner < T , S > { fn drop (& mut self) { while self . dequeue () . is_some () { } } }
    };
}

impl_406!();