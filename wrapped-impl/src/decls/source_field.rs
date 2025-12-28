macro_rules! deps {
    () => {
        MemberUnraw!();
        Field!();
    };
}

macro_rules! source_field {
    () => {
        deps!();
        fn source_field < 'a , 'b > (fields : & 'a [Field < 'b >]) -> Option < & 'a Field < 'b > > { for field in fields { if field . attrs . from . is_some () || field . attrs . source . is_some () { return Some (field) ; } } for field in fields { match & field . member { MemberUnraw :: Named (ident) if ident == "source" => return Some (field) , _ => { } } } None }
    };
}

source_field!()