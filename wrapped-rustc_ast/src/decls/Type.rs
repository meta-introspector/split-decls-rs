macro_rules! deps {
    () => {
        TypeTree!();
        Kind!();
    };
}

macro_rules! Type {
    () => {
        deps!();
        # [derive (Clone , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Type { pub offset : isize , pub size : usize , pub kind : Kind , pub child : TypeTree , }
    };
}

Type!()