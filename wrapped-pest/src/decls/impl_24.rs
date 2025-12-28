macro_rules! deps {
    () => {
        RuleType!();
        QueueableToken!();
        FlatPairs!();
        Tokens!();
        Token!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'i , R : RuleType > FlatPairs < 'i , R > { # [doc = " Returns the `Tokens` for these pairs."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::rc::Rc;"] # [doc = " # use pest;"] # [doc = " # #[allow(non_camel_case_types)]"] # [doc = " # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]"] # [doc = " enum Rule {"] # [doc = "     a"] # [doc = " }"] # [doc = ""] # [doc = " let input = \"\";"] # [doc = " let pairs = pest::state(input, |state| {"] # [doc = "     // generating Token pair with Rule::a ..."] # [doc = " #     state.rule(Rule::a, |s| Ok(s))"] # [doc = " }).unwrap();"] # [doc = " let tokens: Vec<_> = pairs.flatten().tokens().collect();"] # [doc = ""] # [doc = " assert_eq!(tokens.len(), 2);"] # [doc = " ```"] # [inline] pub fn tokens (self) -> Tokens < 'i , R > { tokens :: new (self . queue , self . input , self . start , self . end) } fn next_start (& mut self) { self . start += 1 ; while self . start < self . end && ! self . is_start (self . start) { self . start += 1 ; } } fn next_start_from_end (& mut self) { self . end -= 1 ; while self . end >= self . start && ! self . is_start (self . end) { self . end -= 1 ; } } fn is_start (& self , index : usize) -> bool { match self . queue [index] { QueueableToken :: Start { .. } => true , QueueableToken :: End { .. } => false , } } }
    };
}

impl_24!();