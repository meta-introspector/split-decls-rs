macro_rules! deps {
    () => {
        WriteStyle!();
    };
}

macro_rules! parse_write_style {
    () => {
        deps!();
        fn parse_write_style (spec : & str) -> WriteStyle { match spec { "auto" => WriteStyle :: Auto , "always" => WriteStyle :: Always , "never" => WriteStyle :: Never , _ => Default :: default () , } }
    };
}

parse_write_style!()