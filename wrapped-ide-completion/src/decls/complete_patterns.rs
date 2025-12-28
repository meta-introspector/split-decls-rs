macro_rules! deps {
    () => {
        CompletionContext!();
        PatternContext!();
        Completions!();
    };
}

macro_rules! complete_patterns {
    () => {
        deps!();
        fn complete_patterns (acc : & mut Completions , ctx : & CompletionContext < '_ > , pattern_ctx : & PatternContext ,) { flyimport :: import_on_the_fly_pat (acc , ctx , pattern_ctx) ; fn_param :: complete_fn_param (acc , ctx , pattern_ctx) ; pattern :: complete_pattern (acc , ctx , pattern_ctx) ; record :: complete_record_pattern_fields (acc , ctx , pattern_ctx) ; }
    };
}

complete_patterns!();