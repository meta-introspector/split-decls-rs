macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! UnweightedList {
    () => {
        deps!();
        # [doc = " A very simple adjacency list with no node or label weights."] pub type UnweightedList < Ix > = List < () , Ix > ;
    };
}

UnweightedList!();