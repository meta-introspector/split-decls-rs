macro_rules! deps {
    () => {
        RefSpecRef!();
    };
}

macro_rules! MatchGroup {
    () => {
        deps!();
        # [doc = " A match group is able to match a list of ref specs in order while handling negation, conflicts and one to many mappings."] # [derive (Default , Debug , Clone)] pub struct MatchGroup < 'a > { # [doc = " The specs that take part in item matching."] pub specs : Vec < RefSpecRef < 'a > > , }
    };
}

MatchGroup!();