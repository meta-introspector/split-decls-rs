macro_rules! deps {
    () => {
        UnusedDefSuggestion!();
        LateContext!();
    };
}

macro_rules! UnusedDef {
    () => {
        deps!();
        pub (crate) struct UnusedDef < 'a , 'b > { pub pre : & 'a str , pub post : & 'a str , pub cx : & 'a LateContext < 'b > , pub def_id : DefId , pub note : Option < Symbol > , pub suggestion : Option < UnusedDefSuggestion > , }
    };
}

UnusedDef!()