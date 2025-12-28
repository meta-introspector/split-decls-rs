macro_rules! deps {
    () => {
        MessageLevel!();
    };
}

macro_rules! format_level_column {
    () => {
        deps!();
        fn format_level_column (level : MessageLevel) -> & 'static str { use MessageLevel :: * ; match level { Info => "info" , Failure => "fail" , Success => "done" , } }
    };
}

format_level_column!();