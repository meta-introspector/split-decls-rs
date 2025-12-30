// Generated macro for actual_listing (function)
macro_rules! Depcrate_listing_testsactual_listing {
() => {
// Module: crate::listing::tests
// Provides: {"actual_listing"}
// Dependencies: {}
# [test] fn actual_listing () { let result = rewrite_listing (r#"Now open the *main.rs* file you just created and enter the code in Listing 1-1.

<Listing number="1-1" file-name="main.rs" caption="A program that prints `Hello, world!`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

Save the file and go back to your terminal window"# , Mode :: Default ,) ; assert ! (result . is_ok ()) ; assert_eq ! (result . unwrap () , r##"Now open the *main.rs* file you just created and enter the code in Listing 1-1.

<figure class="listing" id="listing-1-1">
<span class="file-name">Filename: main.rs</span>

````rust
fn main() {
    println!("Hello, world!");
}
````

<figcaption><a href="#listing-1-1">Listing 1-1</a>: A program that prints <code>Hello, world!</code></figcaption>
</figure>

Save the file and go back to your terminal window"##) ; }
};
}
