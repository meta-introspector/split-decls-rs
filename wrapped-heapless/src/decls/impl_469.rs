macro_rules! deps {
    () => {
        Storage!();
        QueueInner!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < T , S : Storage > Drop for QueueInner < T , S > { fn drop (& mut self) { for item in self { unsafe { ptr :: drop_in_place (item) ; } } } }
    };
}

impl_469!()