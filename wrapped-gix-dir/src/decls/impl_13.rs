macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Kind { pub (super) fn is_recursable_dir (& self) -> bool { matches ! (self , Kind :: Directory) } # [doc = " Return `true` if this is a directory on disk. Note that this is true for repositories as well."] pub fn is_dir (& self) -> bool { matches ! (self , Kind :: Directory | Kind :: Repository) } }
    };
}

impl_13!();