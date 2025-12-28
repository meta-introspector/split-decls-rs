macro_rules! deps {
    () => {
        GroupInfo!();
    };
}

macro_rules! GroupInfoPatternNames {
    () => {
        deps!();
        # [doc = " An iterator over capturing groups and their names for a specific pattern."] # [doc = ""] # [doc = " This iterator is created by [`GroupInfo::pattern_names`]."] # [doc = ""] # [doc = " The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`"] # [doc = " from which this iterator was created."] # [derive (Clone , Debug)] pub struct GroupInfoPatternNames < 'a > { it : core :: slice :: Iter < 'a , Option < Arc < str > > > , }
    };
}

GroupInfoPatternNames!();