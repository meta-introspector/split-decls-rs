macro_rules! deps {
    () => {
        CodeGenOptLevel!();
        CodeGenOptSize!();
    };
}

macro_rules! to_llvm_opt_settings {
    () => {
        deps!();
        fn to_llvm_opt_settings (cfg : config :: OptLevel) -> (llvm :: CodeGenOptLevel , llvm :: CodeGenOptSize) { use self :: config :: OptLevel :: * ; match cfg { No => (llvm :: CodeGenOptLevel :: None , llvm :: CodeGenOptSizeNone) , Less => (llvm :: CodeGenOptLevel :: Less , llvm :: CodeGenOptSizeNone) , More => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeNone) , Aggressive => (llvm :: CodeGenOptLevel :: Aggressive , llvm :: CodeGenOptSizeNone) , Size => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeDefault) , SizeMin => (llvm :: CodeGenOptLevel :: Default , llvm :: CodeGenOptSizeAggressive) , } }
    };
}

to_llvm_opt_settings!()