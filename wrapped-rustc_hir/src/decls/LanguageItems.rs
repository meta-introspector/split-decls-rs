macro_rules! LanguageItems {
    () => {
        # [doc = " All of the lang items, defined or not."] # [doc = " Defined lang items can come from the current crate or its dependencies."] # [derive (HashStable_Generic , Debug)] pub struct LanguageItems { # [doc = " Mappings from lang items to their possibly found [`DefId`]s."] # [doc = " The index corresponds to the order in [`LangItem`]."] items : [Option < DefId > ; std :: mem :: variant_count :: < LangItem > ()] , reverse_items : FxIndexMap < DefId , LangItem > , # [doc = " Lang items that were not found during collection."] pub missing : Vec < LangItem > , }
    };
}

LanguageItems!();