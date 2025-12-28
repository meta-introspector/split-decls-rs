macro_rules! deps {
    () => {
        Query!();
        CacheControl!();
    };
}

macro_rules! ValidationResult {
    () => {
        deps!();
        # [doc = " Validation results."] # [derive (Debug , Copy , Clone)] pub struct ValidationResult { # [doc = " Cache control"] pub cache_control : CacheControl , # [doc = " Query complexity"] pub complexity : usize , # [doc = " Query depth"] pub depth : usize , }
    };
}

ValidationResult!();