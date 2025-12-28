macro_rules! Is {
    () => {
        pub trait Is < T > { fn is_type (& self) -> bool ; }
    };
}

Is!();