macro_rules! BuiltinDeriveFn {
    () => {
        pub (crate) type BuiltinDeriveFn = fn (& ExtCtxt < '_ > , Span , & MetaItem , & Annotatable , & mut dyn FnMut (Annotatable) , bool) ;
    };
}

BuiltinDeriveFn!()