macro_rules! deps {
    () => {
        Item!();
        Complete!();
        ModuleId!();
        ImportMap!();
    };
}

macro_rules! ImportInfo {
    () => {
        deps!();
        # [doc = " Item import details stored in the `ImportMap`."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct ImportInfo { # [doc = " A name that can be used to import the item, relative to the container."] pub name : Name , # [doc = " The module containing this item."] pub container : ModuleId , # [doc = " Whether this item is annotated with `#[doc(hidden)]`."] pub is_doc_hidden : bool , # [doc = " Whether this item is annotated with `#[unstable(..)]`."] pub is_unstable : bool , # [doc = " The value of `#[rust_analyzer::completions(...)]`, if exists."] pub complete : Complete , }
    };
}

ImportInfo!()