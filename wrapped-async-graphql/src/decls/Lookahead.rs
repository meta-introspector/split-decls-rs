macro_rules! deps {
    () => {
        Field!();
        Context!();
    };
}

macro_rules! Lookahead {
    () => {
        deps!();
        # [doc = " A selection performed by a query."] pub struct Lookahead < 'a > { fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , fields : Vec < & 'a Field > , context : & 'a Context < 'a > , }
    };
}

Lookahead!()