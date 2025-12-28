macro_rules! GroupingMap {
    () => {
        # [doc = " `GroupingMap` is an intermediate struct for efficient group-and-fold operations."] # [doc = " It groups elements by their key and at the same time fold each group"] # [doc = " using some aggregating operation."] # [doc = ""] # [doc = " No method on this struct performs temporary allocations."] # [derive (Clone , Debug)] # [must_use = "GroupingMap is lazy and do nothing unless consumed"] pub struct GroupingMap < I > { iter : I , }
    };
}

GroupingMap!();