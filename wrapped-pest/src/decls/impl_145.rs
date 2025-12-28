macro_rules! deps {
    () => {
        RuleType!();
        Operator!();
        Assoc!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < R : RuleType > Operator < R > { # [doc = " Creates a new `Operator` from a `Rule` and `Assoc`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use pest::prec_climber::{Assoc, Operator};"] # [doc = " # #[allow(non_camel_case_types)]"] # [doc = " # #[allow(dead_code)]"] # [doc = " # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]"] # [doc = " # enum Rule {"] # [doc = " #     plus,"] # [doc = " #     minus"] # [doc = " # }"] # [doc = " Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Right);"] # [doc = " ```"] pub fn new (rule : R , assoc : Assoc) -> Operator < R > { Operator { rule , assoc , next : None , } } }
    };
}

impl_145!()