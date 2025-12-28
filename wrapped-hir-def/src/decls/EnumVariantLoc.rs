macro_rules! EnumVariantLoc {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct EnumVariantLoc { pub id : AstId < ast :: Variant > , pub parent : EnumId , pub index : u32 , }
    };
}

EnumVariantLoc!()