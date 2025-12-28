macro_rules! deps {
    () => {
        OptimizedExpr!();
        OptimizedRule!();
    };
}

macro_rules! macro_33 {
    () => {
        deps!();
        to_hash_map ! (to_optimized_hash_map , OptimizedRule , OptimizedExpr) ;
    };
}

macro_33!();