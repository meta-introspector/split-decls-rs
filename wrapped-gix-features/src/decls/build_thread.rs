macro_rules! build_thread {
    () => {
        # [doc = " Create a builder for threads which allows them to be spawned into a scope and configured prior to spawning."] pub fn build_thread () -> std :: thread :: Builder { std :: thread :: Builder :: new () }
    };
}

build_thread!()