macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! AsName {
    () => {
        deps!();
        pub trait AsName { fn as_name (& self) -> Name ; }
    };
}

AsName!();