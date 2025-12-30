// Generated macro for fixup_end_of_definition_list (function)
macro_rules! Depcrate_firstpassfixup_end_of_definition_list {
() => {
// Module: crate::firstpass
// Provides: {"fixup_end_of_definition_list"}
// Dependencies: {}
fn fixup_end_of_definition_list (tree : & mut Tree < Item > , list_ix : TreeIndex) { let mut list_item = tree [list_ix] . child ; let mut previous_list_item = None ; while let Some (listitem_ix) = list_item { match & mut tree [listitem_ix] . item . body { ItemBody :: DefinitionListTitle | ItemBody :: DefinitionListDefinition (_) => { previous_list_item = list_item ; list_item = tree [listitem_ix] . next ; } body @ ItemBody :: MaybeDefinitionListTitle => { * body = ItemBody :: Paragraph ; break ; } _ => break , } } if let Some (previous_list_item) = previous_list_item { tree . truncate_to_parent (previous_list_item) ; } }
};
}
