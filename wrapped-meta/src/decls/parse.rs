macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        # [doc = " A helper that will parse using the pest grammar"] # [allow (clippy :: perf)] pub fn parse (rule : Rule , data : & str) -> Result < Pairs < '_ , Rule > , Error < Rule > > { PestParser :: parse (rule , data) }
    };
}

parse!()