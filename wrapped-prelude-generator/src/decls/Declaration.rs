macro_rules! Declaration {
    () => {
        pub trait Declaration { fn name (& self) -> & str ; }
    };
}

Declaration!()