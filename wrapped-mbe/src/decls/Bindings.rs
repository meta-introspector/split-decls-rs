macro_rules! deps {
    () => {
        Binding!();
    };
}

macro_rules! Bindings {
    () => {
        deps!();
        # [doc = " The actual algorithm for expansion is not too hard, but is pretty tricky."] # [doc = " `Bindings` structure is the key to understanding what we are doing here."] # [doc = ""] # [doc = " On the high level, it stores mapping from meta variables to the bits of"] # [doc = " syntax it should be substituted with. For example, if `$e:expr` is matched"] # [doc = " with `1 + 1` by macro_rules, the `Binding` will store `$e -> 1 + 1`."] # [doc = ""] # [doc = " The tricky bit is dealing with repetitions (`$()*`). Consider this example:"] # [doc = ""] # [doc = " ```not_rust"] # [doc = " macro_rules! foo {"] # [doc = "     ($($ i:ident $($ e:expr),*);*) => {"] # [doc = "         $(fn $ i() { $($ e);*; })*"] # [doc = "     }"] # [doc = " }"] # [doc = " foo! { foo 1,2,3; bar 4,5,6 }"] # [doc = " ```"] # [doc = ""] # [doc = " Here, the `$i` meta variable is matched first with `foo` and then with"] # [doc = " `bar`, and `$e` is matched in turn with `1`, `2`, `3`, `4`, `5`, `6`."] # [doc = ""] # [doc = " To represent such \"multi-mappings\", we use a recursive structures: we map"] # [doc = " variables not to values, but to *lists* of values or other lists (that is,"] # [doc = " to the trees)."] # [doc = ""] # [doc = " For the above example, the bindings would store"] # [doc = ""] # [doc = " ```not_rust"] # [doc = " i -> [foo, bar]"] # [doc = " e -> [[1, 2, 3], [4, 5, 6]]"] # [doc = " ```"] # [doc = ""] # [doc = " We construct `Bindings` in the `match_lhs`. The interesting case is"] # [doc = " `TokenTree::Repeat`, where we use `push_nested` to create the desired"] # [doc = " nesting structure."] # [doc = ""] # [doc = " The other side of the puzzle is `expand_subtree`, where we use the bindings"] # [doc = " to substitute meta variables in the output template. When expanding, we"] # [doc = " maintain a `nesting` stack of indices which tells us which occurrence from"] # [doc = " the `Bindings` we should take. We push to the stack when we enter a"] # [doc = " repetition."] # [doc = ""] # [doc = " In other words, `Bindings` is a *multi* mapping from `Symbol` to"] # [doc = " `tt::TokenTree`, where the index to select a particular `TokenTree` among"] # [doc = " many is not a plain `usize`, but a `&[usize]`."] # [derive (Debug , Default , Clone)] struct Bindings < 'a > { inner : FxHashMap < Symbol , Binding < 'a > > , }
    };
}

Bindings!()