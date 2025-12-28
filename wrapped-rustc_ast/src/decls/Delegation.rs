macro_rules! deps {
    () => {
        QSelf!();
        Block!();
        Walkable!();
        Path!();
    };
}

macro_rules! Delegation {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Delegation { # [doc = " Path resolution id."] pub id : NodeId , pub qself : Option < Box < QSelf > > , pub path : Path , pub ident : Ident , pub rename : Option < Ident > , pub body : Option < Box < Block > > , # [doc = " The item was expanded from a glob delegation item."] pub from_glob : bool , }
    };
}

Delegation!();