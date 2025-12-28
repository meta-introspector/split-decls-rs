macro_rules! deps {
    () => {
        Relation!();
    };
}

macro_rules! to_child {
    () => {
        deps!();
        fn to_child (r : Option < Relation >) -> Option < Relation > { r . map (| r | match r { Relation :: Parent (id) => Relation :: ChildOfParent (id) , Relation :: ChildOfParent (id) => Relation :: ChildOfParent (id) , }) }
    };
}

to_child!();