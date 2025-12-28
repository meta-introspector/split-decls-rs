macro_rules! deps {
    () => {
        PassBuilderOptLevel!();
    };
}

macro_rules! to_pass_builder_opt_level {
    () => {
        deps!();
        fn to_pass_builder_opt_level (cfg : config :: OptLevel) -> llvm :: PassBuilderOptLevel { use config :: OptLevel :: * ; match cfg { No => llvm :: PassBuilderOptLevel :: O0 , Less => llvm :: PassBuilderOptLevel :: O1 , More => llvm :: PassBuilderOptLevel :: O2 , Aggressive => llvm :: PassBuilderOptLevel :: O3 , Size => llvm :: PassBuilderOptLevel :: Os , SizeMin => llvm :: PassBuilderOptLevel :: Oz , } }
    };
}

to_pass_builder_opt_level!()