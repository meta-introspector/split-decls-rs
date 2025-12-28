macro_rules! deps {
    () => {
        SemanticsImpl!();
    };
}

macro_rules! Semantics {
    () => {
        deps!();
        # [doc = " Primary API to get semantic information, like types, from syntax trees."] pub struct Semantics < 'db , DB : ? Sized > { pub db : & 'db DB , imp : SemanticsImpl < 'db > , }
    };
}

Semantics!()