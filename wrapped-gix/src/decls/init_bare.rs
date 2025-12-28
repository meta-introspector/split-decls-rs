macro_rules! deps {
    () => {
        Repository!();
        Path!();
        Error!();
        Kind!();
        Options!();
        ThreadSafeRepository!();
    };
}

macro_rules! init_bare {
    () => {
        deps!();
        # [doc = " See [`ThreadSafeRepository::init()`], but returns a [`Repository`] instead."] # [allow (clippy :: result_large_err)] pub fn init_bare (directory : impl AsRef < std :: path :: Path >) -> Result < Repository , init :: Error > { ThreadSafeRepository :: init (directory , create :: Kind :: Bare , create :: Options :: default ()) . map (Into :: into) }
    };
}

init_bare!();