macro_rules! Lazy {
    () => {
        # [doc = " Mock implementation of `lazy_static::Lazy`."] pub struct Lazy < T > { # [doc (hidden)] pub init : fn () -> T , # [doc (hidden)] pub _p : PhantomData < fn (T) > , }
    };
}

Lazy!();