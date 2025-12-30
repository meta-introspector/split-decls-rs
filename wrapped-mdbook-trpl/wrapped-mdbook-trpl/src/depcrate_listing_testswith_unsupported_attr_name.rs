// Generated macro for with_unsupported_attr_name (function)
macro_rules! Depcrate_listing_testswith_unsupported_attr_name {
() => {
// Module: crate::listing::tests
// Provides: {"with_unsupported_attr_name"}
// Dependencies: {}
# [test] fn with_unsupported_attr_name () { let result = rewrite_listing ("<Listing invalid-attr>

```rust
fn main() {}
```

</Listing>" , Mode :: Default ,) ; assert_eq ! (result , Err (String :: from ("Unsupported attribute name: 'invalid-attr'"))) }
};
}
