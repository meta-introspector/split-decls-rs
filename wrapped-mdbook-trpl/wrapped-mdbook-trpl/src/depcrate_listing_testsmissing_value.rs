// Generated macro for missing_value (module)
macro_rules! Depcrate_listing_testsmissing_value {
() => {
// Module: crate::listing::tests
// Provides: {"missing_value"}
// Dependencies: {}
# [cfg (test)] mod missing_value { use super :: * ; # [test] fn for_number () { let result = rewrite_listing (r#"<Listing number>

```rust
fn main() {}
```

</Listing>"# , Mode :: Default ,) ; assert_eq ! (result , Err (String :: from ("Missing value for attribute: 'number'"))) } # [test] fn for_caption () { let result = rewrite_listing (r#"<Listing caption>

```rust
fn main() {}
```

</Listing>"# , Mode :: Default ,) ; assert_eq ! (result , Err (String :: from ("Missing value for attribute: 'caption'"))) } # [test] fn for_file_name () { let result = rewrite_listing (r#"<Listing file-name>

```rust
fn main() {}
```

</Listing>"# , Mode :: Default ,) ; assert_eq ! (result , Err (String :: from ("Missing value for attribute: 'file-name'"))) } }
};
}
