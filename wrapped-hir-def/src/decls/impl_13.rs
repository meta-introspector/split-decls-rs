macro_rules! deps {
    () => {
        AttrsWithOwner!();
        Attrs!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl ops :: Deref for AttrsWithOwner { type Target = Attrs ; fn deref (& self) -> & Attrs { & self . attrs } }
    };
}

impl_13!();