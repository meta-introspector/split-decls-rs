macro_rules! deps {
    () => {
        PrintAttribute!();
        RustcVersion!();
    };
}

macro_rules! CfgEntry {
    () => {
        deps!();
        # [derive (Encodable , Decodable , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum CfgEntry { All (ThinVec < CfgEntry > , Span) , Any (ThinVec < CfgEntry > , Span) , Not (Box < CfgEntry > , Span) , Bool (bool , Span) , NameValue { name : Symbol , name_span : Span , value : Option < (Symbol , Span) > , span : Span } , Version (Option < RustcVersion > , Span) , }
    };
}

CfgEntry!()