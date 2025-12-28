macro_rules! deps {
    () => {
        WaitGroup!();
        Inner!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl Default for WaitGroup { fn default () -> Self { Self { inner : Arc :: new (Inner { cvar : Condvar :: new () , count : Mutex :: new (1) , }) , } } }
    };
}

impl_145!()