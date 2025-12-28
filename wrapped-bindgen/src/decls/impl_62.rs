macro_rules! deps {
    () => {
        Filter!();
        ReferenceStage!();
        Reference!();
        References!();
        Reader!();
        TypeName!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl References { pub fn new (reader : & Reader , stage : Vec < ReferenceStage >) -> Self { Self (stage . into_iter () . map (| stage | { let filter = Filter :: new (reader , & [& stage . path] , & []) ; Reference { name : stage . name , style : stage . style , filter , } }) . collect () ,) } pub fn contains (& self , name : TypeName) -> Option < & Reference > { self . 0 . iter () . find (| reference | reference . filter . includes_type_name (name)) } }
    };
}

impl_62!()