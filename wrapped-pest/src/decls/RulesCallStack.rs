macro_rules! deps {
    () => {
        ParseAttempt!();
    };
}

macro_rules! RulesCallStack {
    () => {
        deps!();
        # [doc = " Rules call stack."] # [doc = " Contains sequence of rule calls that resulted in new parsing attempt."] # [derive (Debug , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct RulesCallStack < R > { # [doc = " Deepest rule caused a parsing error (ParseAttempt::Token transformed into a rule)."] pub deepest : ParseAttempt < R > , # [doc = " Most top rule covering `deepest`."] pub parent : Option < R > , }
    };
}

RulesCallStack!();