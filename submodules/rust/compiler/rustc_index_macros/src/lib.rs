mkuse!{use proc_macro :: TokenStream ;}
mkmod!{newtype, { 
                getname!(newtype);
                getsrc!(newtype);
                getpath!(newtype);
                get_deps!(newtype);
                get_crates!(newtype);
                mkinclude!(newtype);
                 
            }}

macro_rules! newtype_index_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function newtype_index in module {}", module_path!());
    };
}

mkfn!{
    newtype_index_introspect!();
    # [doc = " Creates a struct type `S` that can be used as an index with"] # [doc = " `IndexVec` and so on."] # [doc = ""] # [doc = " There are two ways of interacting with these indices:"] # [doc = ""] # [doc = " - The `From` impls are the preferred way. So you can do"] # [doc = "   `S::from(v)` with a `usize` or `u32`. And you can convert back"] # [doc = "   to an integer with `u32::from(s)`."] # [doc = ""] # [doc = " - Alternatively, you can use the methods `S::new(v)` and `s.index()`"] # [doc = "   to create/return a value."] # [doc = ""] # [doc = " Internally, the index uses a u32, so the index must not exceed"] # [doc = " `u32::MAX`."] # [doc = ""] # [doc = " The impls provided by default are Clone, Copy, PartialEq, Eq, and Hash."] # [doc = ""] # [doc = " Accepted attributes for customization:"] # [doc = " - `#[derive(HashStable_Generic)]`/`#[derive(HashStable)]`: derives"] # [doc = "   `HashStable`, as normal."] # [doc = " - `#[encodable]`: derives `Encodable`/`Decodable`."] # [doc = " - `#[orderable]`: derives `PartialOrd`/`Ord`, plus step-related methods."] # [doc = " - `#[debug_format = \"Foo({})\"]`: derives `Debug` with particular output."] # [doc = " - `#[max = 0xFFFF_FFFD]`: specifies the max value, which allows niche"] # [doc = "   optimizations. The default max value is 0xFFFF_FF00."] # [doc = " - `#[gate_rustc_only]`: makes parts of the generated code nightly-only."] # [proc_macro] # [cfg_attr (feature = "nightly" , allow_internal_unstable (step_trait , rustc_attrs , trusted_step))] pub fn newtype_index (input : TokenStream) -> TokenStream { newtype :: newtype (input) }
}