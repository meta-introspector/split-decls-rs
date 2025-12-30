// Generated macro for no_filename (function)
macro_rules! Depcrate_listing_testsno_filename {
() => {
// Module: crate::listing::tests
// Provides: {"no_filename"}
// Dependencies: {}
# [test] fn no_filename () { let result = rewrite_listing (r#"This is the opening.

<Listing number="1-1" caption="This is the caption">

```rust
fn main() {}
```

</Listing>

This is the closing."# , Mode :: Default ,) ; assert ! (result . is_ok ()) ; assert_eq ! (result . unwrap () , r##"This is the opening.

<figure class="listing" id="listing-1-1">

````rust
fn main() {}
````

<figcaption><a href="#listing-1-1">Listing 1-1</a>: This is the caption</figcaption>
</figure>

This is the closing."##) ; }
};
}
