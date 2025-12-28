macro_rules! deps {
    () => {
        RuleType!();
        Tokens!();
        QueueableToken!();
    };
}

macro_rules! new {
    () => {
        deps!();
        pub fn new < 'i , R : RuleType > (queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , start : usize , end : usize ,) -> Tokens < 'i , R > { if cfg ! (debug_assertions) { for tok in queue . iter () { match * tok { QueueableToken :: Start { input_pos , .. } | QueueableToken :: End { input_pos , .. } => { assert ! (input . get (input_pos ..) . is_some () , "💥 INVALID `Tokens` CREATED 💥") } } } } Tokens { queue , input , start , end , } }
    };
}

new!();