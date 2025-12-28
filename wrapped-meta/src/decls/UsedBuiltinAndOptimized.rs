macro_rules! deps {
    () => {
        OptimizedRule!();
    };
}

macro_rules! UsedBuiltinAndOptimized {
    () => {
        deps!();
        # [doc = " A tuple returned by the validation and processing of the parsed grammar."] # [doc = " The first element is the vector of used builtin rule names,"] # [doc = " the second element is the vector of optimized rules."] type UsedBuiltinAndOptimized < 'i > = (Vec < & 'i str > , Vec < optimizer :: OptimizedRule >) ;
    };
}

UsedBuiltinAndOptimized!();