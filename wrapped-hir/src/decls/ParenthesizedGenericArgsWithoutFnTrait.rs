macro_rules! ParenthesizedGenericArgsWithoutFnTrait {
    () => {
        # [derive (Debug)] pub struct ParenthesizedGenericArgsWithoutFnTrait { pub args : InFile < AstPtr < ast :: ParenthesizedArgList > > , }
    };
}

ParenthesizedGenericArgsWithoutFnTrait!();