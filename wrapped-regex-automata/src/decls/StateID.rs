macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! StateID {
    () => {
        deps!();
        # [doc = " The identifier of a finite automaton state, represented by a"] # [doc = " [`SmallIndex`]."] # [doc = ""] # [doc = " Most regex engines in this crate are built on top of finite automata. Each"] # [doc = " state in a finite automaton defines transitions from its state to another."] # [doc = " Those transitions point to other states via their identifiers, i.e., a"] # [doc = " `StateID`. Since finite automata tend to contain many transitions, it is"] # [doc = " much more memory efficient to define state IDs as small indices."] # [doc = ""] # [doc = " See the [`SmallIndex`] type for more information about what it means for"] # [doc = " a state ID to be a \"small index.\""] # [derive (Clone , Copy , Default , Eq , Hash , PartialEq , PartialOrd , Ord)] # [repr (transparent)] pub struct StateID (SmallIndex) ;
    };
}

StateID!()