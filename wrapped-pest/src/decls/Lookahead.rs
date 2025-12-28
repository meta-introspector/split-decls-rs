macro_rules! deps {
    () => {
        ParserState!();
    };
}

macro_rules! Lookahead {
    () => {
        deps!();
        # [doc = " The current lookahead status of a [`ParserState`]."] # [doc = ""] # [doc = " [`ParserState`]: struct.ParserState.html"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum Lookahead { # [doc = " The positive predicate, written as an ampersand &,"] # [doc = " attempts to match its inner expression."] # [doc = " If the inner expression succeeds, parsing continues,"] # [doc = " but at the same position as the predicate —"] # [doc = " &foo ~ bar is thus a kind of \"AND\" statement:"] # [doc = " \"the input string must match foo AND bar\"."] # [doc = " If the inner expression fails,"] # [doc = " the whole expression fails too."] Positive , # [doc = " The negative predicate, written as an exclamation mark !,"] # [doc = " attempts to match its inner expression."] # [doc = " If the inner expression fails, the predicate succeeds"] # [doc = " and parsing continues at the same position as the predicate."] # [doc = " If the inner expression succeeds, the predicate fails —"] # [doc = " !foo ~ bar is thus a kind of \"NOT\" statement:"] # [doc = " \"the input string must match bar but NOT foo\"."] Negative , # [doc = " No lookahead (i.e. it will consume input)."] None , }
    };
}

Lookahead!();