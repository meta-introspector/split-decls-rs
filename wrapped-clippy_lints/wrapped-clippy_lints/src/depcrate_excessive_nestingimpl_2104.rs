// Generated macro for impl_2104 (impl)
macro_rules! Depcrate_excessive_nestingimpl_2104 {
() => {
// Module: crate::excessive_nesting
// Provides: {"impl_2104"}
// Dependencies: {}
impl Visitor < '_ > for NestingVisitor < '_ , '_ > { fn visit_block (& mut self , block : & Block) { if block . span . from_expansion () { return ; } let snippet = snippet (self . cx , block . span , "{}") . trim () . to_owned () ; if ! snippet . starts_with ('{') || ! snippet . ends_with ('}') { return ; } self . nest_level += 1 ; if ! self . check_indent (block . span , block . id) { walk_block (self , block) ; } self . nest_level -= 1 ; } fn visit_item (& mut self , item : & Item) { if item . span . from_expansion () { return ; } match & item . kind { ItemKind :: Trait (_) | ItemKind :: Impl (_) | ItemKind :: Mod (.. , ModKind :: Loaded (_ , Inline :: Yes , _)) => { self . nest_level += 1 ; if ! self . check_indent (item . span , item . id) { walk_item (self , item) ; } self . nest_level -= 1 ; } , ItemKind :: Mod (..) => walk_item (& mut NestingVisitor { conf : self . conf , cx : self . cx , nest_level : 0 , } , item ,) , _ => walk_item (self , item) , } } }
};
}
