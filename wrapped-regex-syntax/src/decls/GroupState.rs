macro_rules! deps {
    () => {
        Group!();
        Alternation!();
        Concat!();
    };
}

macro_rules! GroupState {
    () => {
        deps!();
        # [doc = " GroupState represents a single stack frame while parsing nested groups"] # [doc = " and alternations. Each frame records the state up to an opening parenthesis"] # [doc = " or a alternating bracket `|`."] # [derive (Clone , Debug)] enum GroupState { # [doc = " This state is pushed whenever an opening group is found."] Group { # [doc = " The concatenation immediately preceding the opening group."] concat : ast :: Concat , # [doc = " The group that has been opened. Its sub-AST is always empty."] group : ast :: Group , # [doc = " Whether this group has the `x` flag enabled or not."] ignore_whitespace : bool , } , # [doc = " This state is pushed whenever a new alternation branch is found. If"] # [doc = " an alternation branch is found and this state is at the top of the"] # [doc = " stack, then this state should be modified to include the new"] # [doc = " alternation."] Alternation (ast :: Alternation) , }
    };
}

GroupState!();