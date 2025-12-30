// Generated macro for StateSet (struct)
macro_rules! Depcrate_dfa_minimizeStateSet {
() => {
// Module: crate::dfa::minimize
// Provides: {"StateSet"}
// Dependencies: {}
# [doc = " A set of states. A state set makes up a single partition in Hopcroft's"] # [doc = " algorithm."] # [doc = ""] # [doc = " It is represented by an ordered set of state identifiers. We use shared"] # [doc = " ownership so that a single state set can be in both the set of partitions"] # [doc = " and in the set of waiting sets simultaneously without an additional"] # [doc = " allocation. Generally, once a state set is built, it becomes immutable."] # [doc = ""] # [doc = " We use this representation because it avoids the overhead of more"] # [doc = " traditional set data structures (HashSet/BTreeSet), and also because"] # [doc = " computing intersection/subtraction on this representation is especially"] # [doc = " fast."] # [derive (Clone , Debug , Eq , PartialEq , PartialOrd , Ord)] struct StateSet { ids : Rc < RefCell < Vec < StateID > > > , }
};
}
