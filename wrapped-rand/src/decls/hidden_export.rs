macro_rules! deps {
    () => {
        IntoFloat!();
    };
}

macro_rules! hidden_export {
    () => {
        deps!();
        # [doc (hidden)] pub mod hidden_export { pub use super :: float :: IntoFloat ; }
    };
}

hidden_export!()