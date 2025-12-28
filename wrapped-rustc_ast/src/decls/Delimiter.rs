macro_rules! deps {
    () => {
        InvisibleOrigin!();
    };
}

macro_rules! Delimiter {
    () => {
        deps!();
        # [doc = " Describes how a sequence of token trees is delimited."] # [doc = " Cannot use `proc_macro::Delimiter` directly because this"] # [doc = " structure should implement some additional traits."] # [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable , HashStable_Generic)] pub enum Delimiter { # [doc = " `( ... )`"] Parenthesis , # [doc = " `{ ... }`"] Brace , # [doc = " `[ ... ]`"] Bracket , # [doc = " `∅ ... ∅`"] # [doc = " An invisible delimiter, that may, for example, appear around tokens coming from a"] # [doc = " \"macro variable\" `$var`. It is important to preserve operator priorities in cases like"] # [doc = " `$var * 3` where `$var` is `1 + 2`."] # [doc = " Invisible delimiters might not survive roundtrip of a token stream through a string."] Invisible (InvisibleOrigin) , }
    };
}

Delimiter!();