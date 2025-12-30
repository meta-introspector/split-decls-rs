// Generated macro for MockTrait (struct)
macro_rules! Depcrate_mock_traitMockTrait {
() => {
// Module: crate::mock_trait
// Provides: {"MockTrait"}
// Dependencies: {}
pub (crate) struct MockTrait { pub attrs : Vec < Attribute > , pub consts : Vec < ImplItemConst > , pub generics : Generics , pub methods : Vec < MockFunction > , # [doc = " Internally-used name of the trait used."] pub ss_name : Ident , # [doc = " Fully-qualified name of the trait"] pub trait_path : Path , # [doc = " Path on which the trait is implemented.  Usually will be the same as"] # [doc = " structname, but might include concrete generic parameters."] self_path : PathSegment , pub types : Vec < ImplItemType > , pub unsafety : Option < Token ! [unsafe] > }
};
}
