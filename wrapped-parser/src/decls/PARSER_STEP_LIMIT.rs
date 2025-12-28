macro_rules! PARSER_STEP_LIMIT {
    () => {
        const PARSER_STEP_LIMIT : usize = if cfg ! (debug_assertions) { 150_000 } else { 15_000_000 } ;
    };
}

PARSER_STEP_LIMIT!();