macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Match < '_ > { # [doc = " Return `true` if the pathspec that matched was negative, which excludes this item from the set."] pub fn is_excluded (& self) -> bool { self . pattern . is_excluded () } }
    };
}

impl_12!();