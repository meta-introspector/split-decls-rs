macro_rules! deps {
    () => {
        GroupInfo!();
        NFA!();
        DFA!();
    };
}

macro_rules! Slots {
    () => {
        deps!();
        # [doc = " The set of epsilon transitions indicating that the current position in a"] # [doc = " search should be saved to a slot."] # [doc = ""] # [doc = " This *only* represents explicit slots. So for example, the pattern"] # [doc = " `[a-z]+([0-9]+)([a-z]+)` has:"] # [doc = ""] # [doc = " * 3 capturing groups, thus 6 slots."] # [doc = " * 1 implicit capturing group, thus 2 implicit slots."] # [doc = " * 2 explicit capturing groups, thus 4 explicit slots."] # [doc = ""] # [doc = " While implicit slots are represented by epsilon transitions in an NFA, we"] # [doc = " do not explicitly represent them here. Instead, implicit slots are assumed"] # [doc = " to be present and handled automatically in the search code. Therefore,"] # [doc = " that means we only need to represent explicit slots in our epsilon"] # [doc = " transitions."] # [doc = ""] # [doc = " Its representation is a bit set. The bit 'i' is set if and only if there"] # [doc = " exists an explicit slot at index 'c', where 'c = (#patterns * 2) + i'. That"] # [doc = " is, the bit 'i' corresponds to the first explicit slot and the first"] # [doc = " explicit slot appears immediately following the last implicit slot. (If"] # [doc = " this is confusing, see `GroupInfo` for more details on how slots works.)"] # [doc = ""] # [doc = " A single `Slots` represents all the active slots in a sub-graph of an NFA,"] # [doc = " where all the states are connected by epsilon transitions. In effect, when"] # [doc = " traversing the one-pass DFA during a search, all slots set in a particular"] # [doc = " transition must be captured by recording the current search position."] # [doc = ""] # [doc = " The API of `Slots` requires the caller to handle the explicit slot offset."] # [doc = " That is, a `Slots` doesn't know where the explicit slots start for a"] # [doc = " particular NFA. Thus, if the callers see's the bit 'i' is set, then they"] # [doc = " need to do the arithmetic above to find 'c', which is the real actual slot"] # [doc = " index in the corresponding NFA."] # [derive (Clone , Copy)] struct Slots (u32) ;
    };
}

Slots!()