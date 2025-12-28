macro_rules! SubstitutionPart {
    () => {
        # [derive (Clone , Debug , PartialEq , Hash , Encodable , Decodable)] pub struct SubstitutionPart { pub span : Span , pub snippet : String , }
    };
}

SubstitutionPart!()