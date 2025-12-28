macro_rules! deps {
    () => {
        SearcherKindFn!();
        Finder!();
        Prefilter!();
    };
}

macro_rules! Searcher {
    () => {
        deps!();
        # [doc = " A \"meta\" substring searcher."] # [doc = ""] # [doc = " To a first approximation, this chooses what it believes to be the \"best\""] # [doc = " substring search implemnetation based on the needle at construction time."] # [doc = " Then, every call to `find` will execute that particular implementation. To"] # [doc = " a second approximation, multiple substring search algorithms may be used,"] # [doc = " depending on the haystack. For example, for supremely short haystacks,"] # [doc = " Rabin-Karp is typically used."] # [doc = ""] # [doc = " See the documentation on `Prefilter` for an explanation of the dispatching"] # [doc = " mechanism. The quick summary is that an enum has too much overhead and"] # [doc = " we can't use dynamic dispatch via traits because we need to work in a"] # [doc = " core-only environment. (Dynamic dispatch works in core-only, but you"] # [doc = " need `&dyn Trait` and we really need a `Box<dyn Trait>` here. The latter"] # [doc = " requires `alloc`.) So instead, we use a union and an appropriately paired"] # [doc = " free function to read from the correct field on the union and execute the"] # [doc = " chosen substring search implementation."] # [derive (Clone)] pub (crate) struct Searcher { call : SearcherKindFn , kind : SearcherKind , rabinkarp : rabinkarp :: Finder , }
    };
}

Searcher!()