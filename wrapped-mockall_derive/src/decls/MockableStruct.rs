macro_rules! MockableStruct {
    () => {
        pub (crate) struct MockableStruct { pub attrs : Vec < Attribute > , pub consts : Vec < ImplItemConst > , pub generics : Generics , # [doc = " Inherent methods of the mockable struct"] pub methods : Vec < ImplItemFn > , pub name : Ident , pub vis : Visibility , pub impls : Vec < ItemImpl > , }
    };
}

MockableStruct!()