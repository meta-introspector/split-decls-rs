macro_rules! deps {
    () => {
        WorkPool!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < W , R > Drop for WorkPool < W , R > { fn drop (& mut self) { self . shutdown_flag . store (true , Ordering :: Release) ; self . work_queue . close () ; } }
    };
}

impl_272!();