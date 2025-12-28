macro_rules! init_bare {
    () => {
        # [doc = " See [`ThreadSafeRepository::init()`], but returns a [`Repository`] instead."] # [allow (clippy :: result_large_err)] pub fn init_bare (directory : impl AsRef < std :: path :: Path >) -> Result < Repository , init :: Error > { ThreadSafeRepository :: init (directory , create :: Kind :: Bare , create :: Options :: default ()) . map (Into :: into) }
    };
}

init_bare!()