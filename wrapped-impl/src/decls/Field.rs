macro_rules! deps {
    () => {
        MemberUnraw!();
        Attrs!();
    };
}

macro_rules! Field {
    () => {
        deps!();
        pub struct Field < 'a > { pub original : & 'a syn :: Field , pub attrs : Attrs < 'a > , pub member : MemberUnraw , pub ty : & 'a Type , pub contains_generic : bool , }
    };
}

Field!()