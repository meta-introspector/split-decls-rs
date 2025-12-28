macro_rules! deps {
    () => {
        AttributeExt!();
        Item!();
    };
}

macro_rules! filter_by_name {
    () => {
        deps!();
        pub fn filter_by_name < A : AttributeExt > (attrs : & [A] , name : Symbol) -> impl Iterator < Item = & A > { attrs . iter () . filter (move | attr | attr . has_name (name)) }
    };
}

filter_by_name!()