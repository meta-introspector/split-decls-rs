macro_rules! deps {
    () => {
        CodegenCx!();
        AttributeKind!();
    };
}

macro_rules! function_return_attr {
    () => {
        deps!();
        fn function_return_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { let function_return_attr = match cx . sess () . opts . unstable_opts . function_return { FunctionReturn :: Keep => return None , FunctionReturn :: ThunkExtern => AttributeKind :: FnRetThunkExtern , } ; Some (function_return_attr . create_attr (cx . llcx)) }
    };
}

function_return_attr!();