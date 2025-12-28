macro_rules! SearchScope {
    () => {
        # [doc = " Generally, `search_scope` returns files that might contain references for the element."] # [doc = " For `pub(crate)` things it's a crate, for `pub` things it's a crate and dependant crates."] # [doc = " In some cases, the location of the references is known to within a `TextRange`,"] # [doc = " e.g. for things like local variables."] # [derive (Clone , Debug)] pub struct SearchScope { entries : FxHashMap < EditionedFileId , Option < TextRange > > , }
    };
}

SearchScope!();