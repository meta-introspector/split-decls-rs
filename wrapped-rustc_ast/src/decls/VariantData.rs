macro_rules! deps {
    () => {
        Recovered!();
        FieldDef!();
        Walkable!();
    };
}

macro_rules! VariantData {
    () => {
        deps!();
        # [doc = " Fields and constructor ids of enum variants and structs."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum VariantData { # [doc = " Struct variant."] # [doc = ""] # [doc = " E.g., `Bar { .. }` as in `enum Foo { Bar { .. } }`."] Struct { fields : ThinVec < FieldDef > , recovered : Recovered } , # [doc = " Tuple variant."] # [doc = ""] # [doc = " E.g., `Bar(..)` as in `enum Foo { Bar(..) }`."] Tuple (ThinVec < FieldDef > , NodeId) , # [doc = " Unit variant."] # [doc = ""] # [doc = " E.g., `Bar = ..` as in `enum Foo { Bar = .. }`."] Unit (NodeId) , }
    };
}

VariantData!();