macro_rules! deps {
    () => {
        AttrId!();
        Attr!();
    };
}

macro_rules! attr_source {
    () => {
        deps!();
        # [doc = " Attributes expect the invoking attribute to be stripped"] fn attr_source (invoc_attr_index : AttrId , node : & ast :: Item) -> Option < ast :: Attr > { cov_mark :: hit ! (attribute_macro_attr_censoring) ; collect_attrs (node) . nth (invoc_attr_index . ast_index ()) . and_then (| (_ , attr) | Either :: left (attr)) }
    };
}

attr_source!();