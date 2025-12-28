macro_rules! Compat {
    () => {
        # [doc = " This adapts from `hyper` IO traits to the ones in Tokio."] # [doc = ""] # [doc = " This is currently used by `h2`, and by hyper internal unit tests."] # [derive (Debug)] pub (crate) struct Compat < T > (pub (crate) T) ;
    };
}

Compat!();