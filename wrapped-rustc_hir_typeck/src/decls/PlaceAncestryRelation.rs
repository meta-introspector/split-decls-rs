macro_rules! PlaceAncestryRelation {
    () => {
        # [doc = " Describe the relationship between the paths of two places"] # [doc = " eg:"] # [doc = " - `foo` is ancestor of `foo.bar.baz`"] # [doc = " - `foo.bar.baz` is an descendant of `foo.bar`"] # [doc = " - `foo.bar` and `foo.baz` are divergent"] enum PlaceAncestryRelation { Ancestor , Descendant , SamePlace , Divergent , }
    };
}

PlaceAncestryRelation!()