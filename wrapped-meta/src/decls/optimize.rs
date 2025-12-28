macro_rules! deps {
    () => {
        Rule!();
        OptimizedRule!();
    };
}

macro_rules! optimize {
    () => {
        deps!();
        # [doc = " Takes pest's ASTs and optimizes them"] pub fn optimize (rules : Vec < Rule >) -> Vec < OptimizedRule > { let map = to_hash_map (& rules) ; let optimized : Vec < OptimizedRule > = rules . into_iter () . map (rotator :: rotate) . map (| rule | skipper :: skip (rule , & map)) . map (unroller :: unroll) . map (concatenator :: concatenate) . map (factorizer :: factor) . map (lister :: list) . map (rule_to_optimized_rule) . collect () ; let optimized_map = to_optimized_hash_map (& optimized) ; optimized . into_iter () . map (| rule | restorer :: restore_on_err (rule , & optimized_map)) . collect () }
    };
}

optimize!();