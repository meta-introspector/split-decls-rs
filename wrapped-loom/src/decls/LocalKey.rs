macro_rules! LocalKey {
    () => {
        # [doc = " Mock implementation of `std::thread::LocalKey`."] pub struct LocalKey < T > { # [doc (hidden)] pub init : fn () -> T , # [doc (hidden)] pub _p : PhantomData < fn (T) > , }
    };
}

LocalKey!()