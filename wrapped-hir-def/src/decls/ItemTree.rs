macro_rules! deps {
    () => {
        ItemVisibilities!();
        BigModItem!();
        Item!();
        SmallModItem!();
    };
}

macro_rules! ItemTree {
    () => {
        deps!();
        # [doc = " The item tree of a source file."] # [derive (Debug , Default , Eq , PartialEq)] pub struct ItemTree { top_level : Box < [ModItemId] > , top_attrs : RawAttrs , attrs : FxHashMap < FileAstId < ast :: Item > , RawAttrs > , vis : ItemVisibilities , big_data : FxHashMap < FileAstId < ast :: Item > , BigModItem > , small_data : FxHashMap < FileAstId < ast :: Item > , SmallModItem > , }
    };
}

ItemTree!();