macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! insert {
    () => {
        deps!();
        fn insert (types : & mut HashMap < & 'static str , Vec < Type > > , name : & 'static str , ty : Type) { types . entry (name) . or_default () . push (ty) ; }
    };
}

insert!()