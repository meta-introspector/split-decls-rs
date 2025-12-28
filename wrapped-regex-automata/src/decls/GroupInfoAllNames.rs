macro_rules! deps {
    () => {
        GroupInfoPatternNames!();
        GroupInfo!();
        PatternID!();
    };
}

macro_rules! GroupInfoAllNames {
    () => {
        deps!();
        # [doc = " An iterator over capturing groups and their names for a `GroupInfo`."] # [doc = ""] # [doc = " This iterator is created by [`GroupInfo::all_names`]."] # [doc = ""] # [doc = " The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`"] # [doc = " from which this iterator was created."] # [derive (Debug)] pub struct GroupInfoAllNames < 'a > { group_info : & 'a GroupInfo , pids : PatternIDIter , current_pid : Option < PatternID > , names : Option < core :: iter :: Enumerate < GroupInfoPatternNames < 'a > > > , }
    };
}

GroupInfoAllNames!();