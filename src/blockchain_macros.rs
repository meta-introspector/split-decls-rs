#[macro_export]
macro_rules! mkcontract {
    ($addr:literal) => {
        pub struct Contract { pub address: &'static str }
    };
}