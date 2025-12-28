macro_rules! deps {
    () => {
        Matches!();
    };
}

macro_rules! RegexSearcher {
    () => {
        deps!();
        # [derive (Debug)] pub struct RegexSearcher < 'r , 't > { haystack : & 't str , it : Matches < 'r , 't > , last_step_end : usize , next_match : Option < (usize , usize) > , }
    };
}

RegexSearcher!();