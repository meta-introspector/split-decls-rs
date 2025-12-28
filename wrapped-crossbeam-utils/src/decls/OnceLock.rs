macro_rules! OnceLock {
    () => {
        pub (crate) struct OnceLock < T > { once : Once , value : UnsafeCell < MaybeUninit < T > > , }
    };
}

OnceLock!()