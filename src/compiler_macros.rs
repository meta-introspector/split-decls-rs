#[macro_export]
macro_rules! mkrust {
    () => {
        pub struct RustCompiler { pub version: &'static str }
        pub fn create_rust_universe() -> RustCompiler {
            RustCompiler { version: "1.84.0-step4" }
        }
    };
}