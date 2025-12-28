macro_rules! Empty {
    () => {
        # [doc = " Iterator adaptor for [the `empty()` function]."] # [doc = ""] # [doc = " [the `empty()` function]: empty()"] pub struct Empty < T > { marker : PhantomData < T > , }
    };
}

Empty!();