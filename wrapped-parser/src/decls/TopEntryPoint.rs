macro_rules! TopEntryPoint {
    () => {
        # [doc = " Parse the whole of the input as a given syntactic construct."] # [doc = ""] # [doc = " This covers two main use-cases:"] # [doc = ""] # [doc = "   * Parsing a Rust file."] # [doc = "   * Parsing a result of macro expansion."] # [doc = ""] # [doc = " That is, for something like"] # [doc = ""] # [doc = " ```ignore"] # [doc = " quick_check! {"] # [doc = "    fn prop() {}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " the input to the macro will be parsed with [`PrefixEntryPoint::Item`], and"] # [doc = " the result will be [`TopEntryPoint::MacroItems`]."] # [doc = ""] # [doc = " [`TopEntryPoint::parse`] makes a guarantee that"] # [doc = "   * all input is consumed"] # [doc = "   * the result is a valid tree (there's one root node)"] # [derive (Debug)] pub enum TopEntryPoint { SourceFile , MacroStmts , MacroItems , Pattern , Type , Expr , # [doc = " Edge case -- macros generally don't expand to attributes, with the"] # [doc = " exception of `cfg_attr` which does!"] MetaItem , }
    };
}

TopEntryPoint!()