// Generated macro for MockItemStruct (struct)
macro_rules! Depcrate_mock_item_structMockItemStruct {
() => {
// Module: crate::mock_item_struct
// Provides: {"MockItemStruct"}
// Dependencies: {}
pub (crate) struct MockItemStruct { attrs : Vec < Attribute > , consts : Vec < ImplItemConst > , generics : Generics , # [doc = " Should Mockall generate a Debug implementation?"] auto_debug : bool , # [doc = " Does the original struct have a `new` method?"] has_new : bool , # [doc = " Inherent methods of the mock struct"] methods : Methods , # [doc = " Name of the overall module that holds all of the mock stuff"] modname : Ident , name : Ident , # [doc = " Is this a whole MockStruct or just a substructure for a trait impl?"] traits : Vec < MockTrait > , vis : Visibility , }
};
}
