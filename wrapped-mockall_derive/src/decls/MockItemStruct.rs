macro_rules! deps {
    () => {
        Methods!();
        MockTrait!();
    };
}

macro_rules! MockItemStruct {
    () => {
        deps!();
        pub (crate) struct MockItemStruct { attrs : Vec < Attribute > , consts : Vec < ImplItemConst > , generics : Generics , # [doc = " Should Mockall generate a Debug implementation?"] auto_debug : bool , # [doc = " Does the original struct have a `new` method?"] has_new : bool , # [doc = " Inherent methods of the mock struct"] methods : Methods , # [doc = " Name of the overall module that holds all of the mock stuff"] modname : Ident , name : Ident , # [doc = " Is this a whole MockStruct or just a substructure for a trait impl?"] traits : Vec < MockTrait > , vis : Visibility , }
    };
}

MockItemStruct!()