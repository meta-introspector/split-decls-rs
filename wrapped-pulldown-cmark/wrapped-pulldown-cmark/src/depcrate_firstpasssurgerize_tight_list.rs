// Generated macro for surgerize_tight_list (function)
macro_rules! Depcrate_firstpasssurgerize_tight_list {
() => {
// Module: crate::firstpass
// Provides: {"surgerize_tight_list"}
// Dependencies: {}
fn surgerize_tight_list (tree : & mut Tree < Item > , list_ix : TreeIndex) { let mut list_item = tree [list_ix] . child ; while let Some (listitem_ix) = list_item { let mut node_ix = tree [listitem_ix] . child ; while let Some (node) = node_ix { if let ItemBody :: Paragraph = tree [node] . item . body { tree [node] . item . body = ItemBody :: TightParagraph ; } node_ix = tree [node] . next ; } list_item = tree [listitem_ix] . next ; } }
};
}
