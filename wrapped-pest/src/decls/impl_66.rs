macro_rules! deps {
    () => {
        QueueableToken!();
        Token!();
        Tokens!();
        Position!();
        RuleType!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < 'i , R : RuleType > Tokens < 'i , R > { fn create_token (& self , index : usize) -> Token < 'i , R > { match self . queue [index] { QueueableToken :: Start { end_token_index , input_pos , } => { let rule = match self . queue [end_token_index] { QueueableToken :: End { rule , .. } => rule , _ => unreachable ! () , } ; Token :: Start { rule , pos : position :: Position :: new_internal (self . input , input_pos) , } } QueueableToken :: End { rule , input_pos , .. } => Token :: End { rule , pos : position :: Position :: new_internal (self . input , input_pos) , } , } } }
    };
}

impl_66!();