// Generated macro for without_number (function)
macro_rules! Depcrate_listing_testswithout_number {
() => {
// Module: crate::listing::tests
// Provides: {"without_number"}
// Dependencies: {}
# [test] fn without_number () { let result = rewrite_listing (r#"<Listing file-name="src/main.rs">

```rust
fn main() {}
```

</Listing>"# , Mode :: Default ,) ; assert ! (result . is_ok ()) ; assert_eq ! (result . unwrap () , r#"<figure class="listing">
<span class="file-name">Filename: src/main.rs</span>

````rust
fn main() {}
````

</figure>"#) ; }
};
}
