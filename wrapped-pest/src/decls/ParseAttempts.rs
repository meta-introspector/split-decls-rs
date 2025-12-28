macro_rules! deps {
    () => {
        Tokens!();
        RulesCallStack!();
        ParsingToken!();
    };
}

macro_rules! ParseAttempts {
    () => {
        deps!();
        # [doc = " Structure that tracks all the parsing attempts made on the max position."] # [doc = " We want to give an error hint about parsing rules that succeeded"] # [doc = " at the farthest input position."] # [doc = " The intuition is such rules will be most likely the query user initially wanted to write."] # [derive (Debug , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct ParseAttempts < R > { # [doc = " Indicates whether the parsing attempts are tracked."] enabled : bool , # [doc = " Vec of rule calls sequences awaiting tokens at the same `max_position`."] # [doc = " If there are several stacks in vec, it means all those rule stacks are \"equal\""] # [doc = " because their attempts occurred on the same position."] pub call_stacks : Vec < RulesCallStack < R > > , # [doc = " Tokens that could be putted at `max_position`"] # [doc = " in order to get a valid grammar query."] expected_tokens : Vec < ParsingToken > , # [doc = " Tokens that we've prohibited to be putted at `max_position`"] # [doc = " in order to get a valid grammar query."] unexpected_tokens : Vec < ParsingToken > , # [doc = " Max position at which we were expecting to see one of `expected_tokens`."] pub max_position : usize , }
    };
}

ParseAttempts!();