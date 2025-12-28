macro_rules! deps {
    () => {
        RuleType!();
        Pairs!();
        Error!();
    };
}

macro_rules! Parser {
    () => {
        deps!();
        # [doc = " A trait with a single method that parses strings."] pub trait Parser < R : RuleType > { # [doc = " Parses a `&str` starting from `rule`."] # [allow (clippy :: perf)] fn parse (rule : R , input : & str) -> Result < Pairs < '_ , R > , Error < R > > ; }
    };
}

Parser!();