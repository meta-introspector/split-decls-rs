macro_rules! AssocSearchMode {
    () => {
        # [doc = " Three possible ways to search for the name in associated and/or other items."] # [derive (Debug , Clone , Copy)] pub enum AssocSearchMode { # [doc = " Search for the name in both associated and other items."] Include , # [doc = " Search for the name in other items only."] Exclude , # [doc = " Search for the name in the associated items only."] AssocItemsOnly , }
    };
}

AssocSearchMode!()