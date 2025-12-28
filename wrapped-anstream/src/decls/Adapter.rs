macro_rules! Adapter {
    () => {
        # [doc = " A shim which allows a [`std::io::Write`] to be implemented in terms of a [`std::fmt::Write`]"] # [doc = ""] # [doc = " This saves off I/O errors. instead of discarding them"] pub (crate) struct Adapter < W > where W : FnMut (& [u8]) -> std :: io :: Result < () > , { writer : W , error : std :: io :: Result < () > , }
    };
}

Adapter!()