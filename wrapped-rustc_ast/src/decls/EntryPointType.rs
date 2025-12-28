macro_rules! EntryPointType {
    () => {
        # [derive (Debug)] pub enum EntryPointType { # [doc = " This function is not an entrypoint."] None , # [doc = " This is a function called `main` at the root level."] # [doc = " ```"] # [doc = " fn main() {}"] # [doc = " ```"] MainNamed , # [doc = " This is a function with the `#[rustc_main]` attribute."] # [doc = " Used by the testing harness to create the test entrypoint."] # [doc = " ```ignore (clashes with test entrypoint)"] # [doc = " #[rustc_main]"] # [doc = " fn main() {}"] # [doc = " ```"] RustcMainAttr , # [doc = " This function is **not** an entrypoint but simply named `main` (not at the root)."] # [doc = " This is only used for diagnostics."] # [doc = " ```"] # [doc = " #[allow(dead_code)]"] # [doc = " mod meow {"] # [doc = "     fn main() {}"] # [doc = " }"] # [doc = " ```"] OtherMain , }
    };
}

EntryPointType!();