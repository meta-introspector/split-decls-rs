macro_rules! NakedFunctionsAsmBlock {
    () => {
        pub (crate) struct NakedFunctionsAsmBlock { pub span : Span , pub multiple_asms : Vec < Span > , pub non_asms : Vec < Span > , }
    };
}

NakedFunctionsAsmBlock!()