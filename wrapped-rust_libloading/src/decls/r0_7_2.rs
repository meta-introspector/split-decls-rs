macro_rules! deps {
    () => {
        Symbol!();
        Library!();
    };
}

macro_rules! r0_7_2 {
    () => {
        deps!();
        # [doc = " Release 0.7.2 (2021-11-14)"] # [doc = ""] # [doc = " Cargo.toml now specifies the MSRV bounds, which enables tooling to report an early failure when"] # [doc = " the version of the toolchain is insufficient. Refer to the [min-rust-version RFC] and its"] # [doc = " [tracking issue]."] # [doc = ""] # [doc = " [min-rust-version RFC]: https://rust-lang.github.io/rfcs/2495-min-rust-version.html"] # [doc = " [tracking issue]: https://github.com/rust-lang/rust/issues/65262"] # [doc = ""] # [doc = " Additionally, on platforms `libloading` has no support (today: `not(any(unix, windows))`), we"] # [doc = " will no longer attempt to implement the cross-platform `Library` and `Symbol` types. This makes"] # [doc = " `libloading` compile on targets such as `wasm32-unknown-unknown` and gives ability to the"] # [doc = " downstream consumers of this library to decide how they want to handle the absence of the"] # [doc = " library loading implementation in their code. One of such approaches could be depending on"] # [doc = " `libloading` itself optionally as such:"] # [doc = ""] # [doc = " ```toml"] # [doc = " [target.'cfg(any(unix, windows))'.dependencies.libloading]"] # [doc = " version = \"0.7\""] # [doc = " ```"] pub mod r0_7_2 { }
    };
}

r0_7_2!();