macro_rules! deps {
    () => {
        VersionDef!();
        ByteString!();
    };
}

macro_rules! impl_1164 {
    () => {
        deps!();
        impl < 'data > VersionDef < 'data > { # [doc = " Optimise for the common case where the first version is the same as the base version."] fn is_shared (& self , index : usize , base : Option < & ByteString < '_ > >) -> bool { index == 1 && self . names . len () == 1 && self . names . first () == base } }
    };
}

impl_1164!();