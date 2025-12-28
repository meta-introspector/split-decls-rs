macro_rules! deps {
    () => {
        Execution!();
    };
}

macro_rules! fence_seqcst {
    () => {
        deps!();
        fn fence_seqcst (execution : & mut Execution) { fence_acqrel (execution) ; execution . threads . seq_cst_fence () ; }
    };
}

fence_seqcst!();