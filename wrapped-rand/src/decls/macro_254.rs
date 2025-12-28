macro_rules! deps {
    () => {
        ReseedingRng!();
        OsRng!();
        ThreadRng!();
    };
}

macro_rules! macro_254 {
    () => {
        deps!();
        thread_local ! (static THREAD_RNG_KEY : Rc < UnsafeCell < ReseedingRng < Core , OsRng >>> = { let rng = ReseedingRng :: new (THREAD_RNG_RESEED_THRESHOLD , OsRng) . unwrap_or_else (| err | panic ! ("could not initialize ThreadRng: {}" , err)) ; Rc :: new (UnsafeCell :: new (rng)) }) ;
    };
}

macro_254!()