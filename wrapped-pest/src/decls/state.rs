macro_rules! deps {
    () => {
        Pairs!();
        Error!();
        ParserState!();
        ParseResult!();
        ErrorVariant!();
        Position!();
        RuleType!();
    };
}

macro_rules! state {
    () => {
        deps!();
        # [doc = " Creates a `ParserState` from a `&str`, supplying it to a closure `f`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use pest;"] # [doc = " let input = \"\";"] # [doc = " pest::state::<(), _>(input, |s| Ok(s)).unwrap();"] # [doc = " ```"] # [allow (clippy :: perf)] pub fn state < 'i , R : RuleType , F > (input : & 'i str , f : F) -> Result < pairs :: Pairs < 'i , R > , Error < R > > where F : FnOnce (Box < ParserState < 'i , R > >) -> ParseResult < Box < ParserState < 'i , R > > > , { let state = ParserState :: new (input) ; match f (state) { Ok (state) => { let len = state . queue . len () ; Ok (new (Rc :: new (state . queue) , input , None , 0 , len)) } Err (mut state) => { let variant = if state . reached_call_limit () { ErrorVariant :: CustomError { message : "call limit reached" . to_owned () , } } else { state . pos_attempts . sort () ; state . pos_attempts . dedup () ; state . neg_attempts . sort () ; state . neg_attempts . dedup () ; ErrorVariant :: ParsingError { positives : state . pos_attempts . clone () , negatives : state . neg_attempts . clone () , } } ; if state . parse_attempts . enabled { Err (Error :: new_from_pos_with_parsing_attempts (variant , Position :: new_internal (input , state . attempt_pos) , state . parse_attempts . clone () ,)) } else { Err (Error :: new_from_pos (variant , Position :: new_internal (input , state . attempt_pos) ,)) } } } }
    };
}

state!()