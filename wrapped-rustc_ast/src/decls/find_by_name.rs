macro_rules! deps {
    () => {
        AttributeExt!();
    };
}

macro_rules! find_by_name {
    () => {
        deps!();
        pub fn find_by_name < A : AttributeExt > (attrs : & [A] , name : Symbol) -> Option < & A > { filter_by_name (attrs , name) . next () }
    };
}

find_by_name!();