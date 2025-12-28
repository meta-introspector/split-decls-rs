macro_rules! deps {
    () => {
        AttributeStack!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Deref for AttributeStack < '_ > { type Target = gix_worktree :: Stack ; fn deref (& self) -> & Self :: Target { & self . inner } }
    };
}

impl_38!();