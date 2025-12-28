macro_rules! deps {
    () => {
        AttributePlace!();
        AddFunctionAttributes!();
    };
}

macro_rules! apply_to_llfn {
    () => {
        deps!();
        pub (crate) fn apply_to_llfn (llfn : & Value , idx : AttributePlace , attrs : & [& Attribute]) { if ! attrs . is_empty () { llvm :: AddFunctionAttributes (llfn , idx , attrs) ; } }
    };
}

apply_to_llfn!();