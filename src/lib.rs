pub mod decls;
pub use decls::*;

// COMMENTED OUT: These includes cause 9k+ compilation errors
// include!("output2_modules.rs");

// Include output2 modules if available
// #[cfg(feature = "output2")]
// include!(concat!(env!("OUT_DIR"), "/output2_modules.rs"));

// Fallback: try to include from src if build.rs generated it there
// #[cfg(not(feature = "output2"))]
// mod output2_fallback {
//     include!("output2_modules.rs");
// }
// #[cfg(not(feature = "output2"))]
// pub use output2_fallback::*;
