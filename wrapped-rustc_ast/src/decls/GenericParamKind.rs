macro_rules! deps {
    () => {
        Const!();
        Ty!();
        AnonConst!();
        Lifetime!();
        Type!();
        Walkable!();
    };
}

macro_rules! GenericParamKind {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum GenericParamKind { # [doc = " A lifetime definition (e.g., `'a: 'b + 'c + 'd`)."] Lifetime , Type { default : Option < Box < Ty > > , } , Const { ty : Box < Ty > , # [doc = " Span of the whole parameter definition, including default."] span : Span , # [doc = " Optional default value for the const generic param."] default : Option < AnonConst > , } , }
    };
}

GenericParamKind!()