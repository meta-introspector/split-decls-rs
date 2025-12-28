macro_rules! AutoderefSnapshot {
    () => {
        struct AutoderefSnapshot < 'db , Steps > { at_start : bool , reached_recursion_limit : bool , steps : Steps , cur_ty : Ty < 'db > , obligations : PredicateObligations < 'db > , }
    };
}

AutoderefSnapshot!()