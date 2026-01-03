// Test that rustc_includes.rs compiles successfully
// This validates that all rustc modules are included in correct dependency order

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(internal_features)]
#![feature(rustc_attrs)]

// Include all rustc modules in topological dependency order
include!("rustc_includes.rs");

pub fn test_rustc_compilation() {
    println!("✅ All rustc modules compiled successfully in dependency order!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_includes_compile() {
        test_rustc_compilation();
    }
}
