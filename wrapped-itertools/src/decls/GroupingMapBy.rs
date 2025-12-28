macro_rules! deps {
    () => {
        GroupingMap!();
        MapForGrouping!();
    };
}

macro_rules! GroupingMapBy {
    () => {
        deps!();
        # [doc = " `GroupingMapBy` is an intermediate struct for efficient group-and-fold operations."] # [doc = ""] # [doc = " See [`GroupingMap`] for more information."] pub type GroupingMapBy < I , F > = GroupingMap < MapForGrouping < I , F > > ;
    };
}

GroupingMapBy!()