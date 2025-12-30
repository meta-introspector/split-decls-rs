// Generated macro for with_unsupported_attr_name_with_arg (function)
macro_rules! Depcrate_listing_testswith_unsupported_attr_name_with_arg {
() => {
// Module: crate::listing::tests
// Provides: {"with_unsupported_attr_name_with_arg"}
// Dependencies: {}
# [test] fn with_unsupported_attr_name_with_arg () { let result = rewrite_listing (r#"<Listing invalid-attr="123">

```rust
fn main() {}
```

</Listing>"# , Mode :: Default ,) ; assert_eq ! (result , Err (String :: from ("Unsupported attribute name: 'invalid-attr'"))) }
};
}
