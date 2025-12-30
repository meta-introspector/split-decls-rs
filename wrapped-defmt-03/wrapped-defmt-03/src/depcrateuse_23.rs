// Generated macro for use_23 (pub_use)
macro_rules! Depcrateuse_23 {
() => {
// Module: crate
// Provides: {"use_23"}
// Dependencies: {}
# [doc = " Defines the global defmt logger."] # [doc = ""] # [doc = " `#[global_logger]` needs to be put on a unit struct type declaration. This struct has to"] # [doc = " implement the [`Logger`] trait."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use defmt::{Logger, global_logger};"] # [doc = ""] # [doc = " #[global_logger]"] # [doc = " struct MyLogger;"] # [doc = ""] # [doc = " unsafe impl Logger for MyLogger {"] # [doc = "     fn acquire() {"] # [doc = " # todo!()"] # [doc = "         // ..."] # [doc = "     }"] # [doc = "     unsafe fn flush() {"] # [doc = "         # todo!()"] # [doc = "         // ..."] # [doc = "     }"] # [doc = "     unsafe fn release() {"] # [doc = " # todo!()"] # [doc = "         // ..."] # [doc = "     }"] # [doc = "     unsafe fn write(bytes: &[u8]) {"] # [doc = " # todo!()"] # [doc = "         // ..."] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`Logger`]: trait.Logger.html"] pub use defmt10 :: global_logger ;
};
}
