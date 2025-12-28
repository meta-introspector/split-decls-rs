macro_rules! deps {
    () => {
        Variant!();
        Walkable!();
    };
}

macro_rules! EnumDef {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct EnumDef { pub variants : ThinVec < Variant > , }
    };
}

EnumDef!()