// Generated macro for default_mode_works (function)
macro_rules! Depcrate_listing_testsdefault_mode_works {
() => {
// Module: crate::listing::tests
// Provides: {"default_mode_works"}
// Dependencies: {}
# [doc = " Note: This inserts an additional backtick around the re-emitted code."] # [doc = " It is not clear *why*, but that seems to be an artifact of the rendering"] # [doc = " done by the `pulldown_cmark_to_cmark` crate."] # [test] fn default_mode_works () { let result = rewrite_listing (r#"<Listing number="1-2" caption="A write-up which *might* include inline Markdown like `code` etc." file-name="src/main.rs">

```rust
fn main() {}
```

</Listing>"# , Mode :: Default ,) ; assert_eq ! (& result . unwrap () , r##"<figure class="listing" id="listing-1-2">
<span class="file-name">Filename: src/main.rs</span>

````rust
fn main() {}
````

<figcaption><a href="#listing-1-2">Listing 1-2</a>: A write-up which <em>might</em> include inline Markdown like <code>code</code> etc.</figcaption>
</figure>"##) ; }
};
}
