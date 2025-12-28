macro_rules! deps {
    () => {
        IdleState!();
        JobsEventCounter!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl IdleState { fn wake_fully (& mut self) { self . rounds = 0 ; self . jobs_counter = JobsEventCounter :: DUMMY ; } fn wake_partly (& mut self) { self . rounds = ROUNDS_UNTIL_SLEEPY ; self . jobs_counter = JobsEventCounter :: DUMMY ; } }
    };
}

impl_222!()