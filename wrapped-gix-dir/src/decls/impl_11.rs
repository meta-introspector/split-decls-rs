macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < std :: fs :: FileType > for Kind { fn from (value : FileType) -> Self { if value . is_dir () { Kind :: Directory } else if value . is_symlink () { Kind :: Symlink } else if value . is_file () { Kind :: File } else { Kind :: Untrackable } } }
    };
}

impl_11!()