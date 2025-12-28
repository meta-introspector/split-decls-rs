macro_rules! deps {
    () => {
        GrastTriple!();
    };
}

macro_rules! GrastDb {
    () => {
        deps!();
        # [decl (struct , name = "GrastDb" , vis = "pub" , hash = "508c1233")] pub struct GrastDb { pub triples : Vec < GrastTriple > , pub index : HashMap < String , Vec < usize > > , }
    };
}

GrastDb!();