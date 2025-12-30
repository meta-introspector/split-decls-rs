// Generated macro for listing_with_embedded_angle_brackets (function)
macro_rules! Depcrate_listing_testslisting_with_embedded_angle_brackets {
() => {
// Module: crate::listing::tests
// Provides: {"listing_with_embedded_angle_brackets"}
// Dependencies: {}
# [test] fn listing_with_embedded_angle_brackets () { let result = rewrite_listing (r#"<Listing number="34-5" caption="This has a `Box<T>` in it.">

```rust
fn get_a_box_of<T>(t: T) -> Box<T> {
    Box::new(T)
}
```

</Listing>"# , Mode :: Default ,) ; assert_eq ! (& result . unwrap () , r##"<figure class="listing" id="listing-34-5">

````rust
fn get_a_box_of<T>(t: T) -> Box<T> {
    Box::new(T)
}
````

<figcaption><a href="#listing-34-5">Listing 34-5</a>: This has a <code>Box&lt;T&gt;</code> in it.</figcaption>
</figure>"##) ; }
};
}
