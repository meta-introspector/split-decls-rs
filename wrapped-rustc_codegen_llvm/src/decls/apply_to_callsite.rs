macro_rules! deps {
    () => {
        AttributePlace!();
        AddCallSiteAttributes!();
    };
}

macro_rules! apply_to_callsite {
    () => {
        deps!();
        pub (crate) fn apply_to_callsite (callsite : & Value , idx : AttributePlace , attrs : & [& Attribute]) { if ! attrs . is_empty () { llvm :: AddCallSiteAttributes (callsite , idx , attrs) ; } }
    };
}

apply_to_callsite!();