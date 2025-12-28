macro_rules! deps {
    () => {
        ItemKind!();
    };
}

macro_rules! Item {
    () => {
        deps!();
        # [doc = " An item"] # [doc = ""] # [doc = " For more details, see the [rust lang reference]."] # [doc = " Note that the reference does not document nightly-only features."] # [doc = " There may be also slight differences in the names and representation of AST nodes between"] # [doc = " the compiler and the reference."] # [doc = ""] # [doc = " [rust lang reference]: https://doc.rust-lang.org/reference/items.html"] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Item < 'hir > { pub owner_id : OwnerId , pub kind : ItemKind < 'hir > , pub span : Span , pub vis_span : Span , pub has_delayed_lints : bool , }
    };
}

Item!();