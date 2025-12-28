macro_rules! deps {
    () => {
        Relation!();
    };
}

macro_rules! TreeInfoTuple {
    () => {
        deps!();
        type TreeInfoTuple = (Option < ObjectId > , Option < ObjectId > , Option < Relation >) ;
    };
}

TreeInfoTuple!();