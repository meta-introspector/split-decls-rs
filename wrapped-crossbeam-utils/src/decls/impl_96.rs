macro_rules! deps {
    () => {
        Parker!();
        Unparker!();
        Inner!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Default for Parker { fn default () -> Self { Self { unparker : Unparker { inner : Arc :: new (Inner { state : AtomicUsize :: new (EMPTY) , lock : Mutex :: new (()) , cvar : Condvar :: new () , }) , } , _marker : PhantomData , } } }
    };
}

impl_96!()