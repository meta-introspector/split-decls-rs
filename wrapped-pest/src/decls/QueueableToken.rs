macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! QueueableToken {
    () => {
        deps!();
        # [derive (Debug)] pub enum QueueableToken < 'i , R > { Start { # [doc = " Queue (as a vec) contains both `Start` token and `End` for the same rule."] # [doc = " This field is an index of corresponding `End` token in vec."] end_token_index : usize , # [doc = " Position from which rule was tried to parse (or successfully parsed)."] input_pos : usize , } , End { # [doc = " Queue (as a vec) contains both `Start` token and `End` for the same rule."] # [doc = " This filed is an index of corresponding `Start` token in vec."] start_token_index : usize , rule : R , tag : Option < & 'i str > , # [doc = " Position at which successfully parsed rule finished (ended)."] input_pos : usize , } , }
    };
}

QueueableToken!();