macro_rules! deps {
    () => {
        Writer!();
        FormatFn!();
        Builder!();
    };
}

macro_rules! Logger {
    () => {
        deps!();
        # [doc = " The env logger."] # [doc = ""] # [doc = " This struct implements the `Log` trait from the [`log` crate][log-crate-url],"] # [doc = " which allows it to act as a logger."] # [doc = ""] # [doc = " The [`init()`], [`try_init()`], [`Builder::init()`] and [`Builder::try_init()`]"] # [doc = " methods will each construct a `Logger` and immediately initialize it as the"] # [doc = " default global logger."] # [doc = ""] # [doc = " If you'd instead need access to the constructed `Logger`, you can use"] # [doc = " the associated [`Builder`] and install it with the"] # [doc = " [`log` crate][log-crate-url] directly."] # [doc = ""] # [doc = " [log-crate-url]: https://docs.rs/log"] # [doc = " [`init()`]: fn.init.html"] # [doc = " [`try_init()`]: fn.try_init.html"] # [doc = " [`Builder::init()`]: struct.Builder.html#method.init"] # [doc = " [`Builder::try_init()`]: struct.Builder.html#method.try_init"] # [doc = " [`Builder`]: struct.Builder.html"] pub struct Logger { writer : Writer , filter : env_filter :: Filter , format : FormatFn , }
    };
}

Logger!()