macro_rules! deps {
    () => {
        TargetDataLayout!();
    };
}

macro_rules! HasDataLayout {
    () => {
        deps!();
        pub trait HasDataLayout { fn data_layout (& self) -> & TargetDataLayout ; }
    };
}

HasDataLayout!()