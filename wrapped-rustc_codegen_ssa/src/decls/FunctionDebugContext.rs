macro_rules! deps {
    () => {
        DebugScope!();
    };
}

macro_rules! FunctionDebugContext {
    () => {
        deps!();
        pub struct FunctionDebugContext < 'tcx , S , L > { # [doc = " Maps from source code to the corresponding debug info scope."] pub scopes : IndexVec < mir :: SourceScope , DebugScope < S , L > > , # [doc = " Maps from an inlined function to its debug info declaration."] pub inlined_function_scopes : FxHashMap < Instance < 'tcx > , S > , }
    };
}

FunctionDebugContext!();