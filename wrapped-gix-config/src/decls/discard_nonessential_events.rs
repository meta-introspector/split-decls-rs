macro_rules! deps {
    () => {
        Comment!();
        Event!();
        Whitespace!();
    };
}

macro_rules! discard_nonessential_events {
    () => {
        deps!();
        fn discard_nonessential_events (e : & Event < '_ >) -> bool { match e { Event :: Whitespace (_) | Event :: Comment (_) | Event :: Newline (_) => false , Event :: SectionHeader (_) | Event :: SectionValueName (_) | Event :: KeyValueSeparator | Event :: Value (_) | Event :: ValueNotDone (_) | Event :: ValueDone (_) => true , } }
    };
}

discard_nonessential_events!()