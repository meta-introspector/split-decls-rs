macro_rules! deps {
    () => {
        Methods!();
    };
}

macro_rules! MockItemTraitImpl {
    () => {
        deps!();
        pub (crate) struct MockItemTraitImpl { attrs : Vec < Attribute > , generics : Generics , # [doc = " Inherent methods of the mock struct"] methods : Methods , # [doc = " Name of the overall module that holds all of the mock stuff"] modname : Ident , name : Ident , # [doc = " Name of the field of this type in the parent's structure"] fieldname : Ident , }
    };
}

MockItemTraitImpl!()