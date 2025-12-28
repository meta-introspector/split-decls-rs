macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! SsrMatches {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct SsrMatches { pub matches : Vec < Match > , }
    };
}

SsrMatches!()