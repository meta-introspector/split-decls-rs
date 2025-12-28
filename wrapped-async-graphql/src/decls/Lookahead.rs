macro_rules! deps {
    () => {
        Context!();
        Field!();
    };
}

macro_rules! Lookahead {
    () => {
        deps!();
        # [doc = " A selection performed by a query."] pub struct Lookahead < 'a > { fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , fields : Vec < & 'a Field > , context : & 'a Context < 'a > , }
    };
}

Lookahead!();