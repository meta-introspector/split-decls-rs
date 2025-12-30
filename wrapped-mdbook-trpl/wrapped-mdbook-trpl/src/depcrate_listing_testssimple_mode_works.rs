// Generated macro for simple_mode_works (function)
macro_rules! Depcrate_listing_testssimple_mode_works {
() => {
// Module: crate::listing::tests
// Provides: {"simple_mode_works"}
// Dependencies: {}
# [test] fn simple_mode_works () { let result = rewrite_listing (r#"Leading text.

<Listing number="1-2" caption="A write-up which *might* include inline Markdown like `code` etc." file-name="src/main.rs">

```rust
fn main() {}
```

</Listing>

Trailing text."# , Mode :: Simple ,) ; assert_eq ! (& result . unwrap () , r#"Leading text.

src/main.rs

```rust
fn main() {}
```

Listing 1-2: A write-up which *might* include inline Markdown like `code` etc.

Trailing text."#) ; }
};
}
