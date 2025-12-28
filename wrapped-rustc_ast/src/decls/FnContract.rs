macro_rules! deps {
    () => {
        Walkable!();
        Expr!();
    };
}

macro_rules! FnContract {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct FnContract { pub requires : Option < Box < Expr > > , pub ensures : Option < Box < Expr > > , }
    };
}

FnContract!()