macro_rules! deps {
    () => {
        InFile!();
        MacroCallId!();
    };
}

macro_rules! EagerCallBackFn {
    () => {
        deps!();
        pub type EagerCallBackFn < 'a > = & 'a mut dyn FnMut (InFile < (syntax :: AstPtr < ast :: MacroCall > , span :: FileAstId < ast :: MacroCall >) > , MacroCallId ,) ;
    };
}

EagerCallBackFn!()