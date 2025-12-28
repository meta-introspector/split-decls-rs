macro_rules! deps {
    () => {
        AutoLogParse!();
        AutoParseRecursion!();
    };
}

macro_rules! try_begin_parse {
    () => {
        deps!();
        # [doc = " Performs the two operations that begin every parse:"] # [doc = ""] # [doc = " 1. Keeps track of recursion levels and early returns with an error if there"] # [doc = "    is too much recursion."] # [doc = ""] # [doc = " 2. Automatically log start and end parsing in an s-expression format, when the"] # [doc = "    `logging` feature is enabled."] macro_rules ! try_begin_parse { ($ production : expr , $ ctx : expr , $ input : expr) => { let _log = AutoLogParse :: new ($ production , $ input) ; let _auto_check_recursion = AutoParseRecursion :: new ($ ctx) ?; } ; }
    };
}

try_begin_parse!();