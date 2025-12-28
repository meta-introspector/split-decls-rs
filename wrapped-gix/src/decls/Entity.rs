macro_rules! deps {
    () => {
        Clone!();
        String!();
    };
}

macro_rules! Entity {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub (crate) struct Entity { pub name : Option < BString > , pub email : Option < BString > , pub time : Option < String > , }
    };
}

Entity!()