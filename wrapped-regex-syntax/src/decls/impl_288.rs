macro_rules! deps {
    () => {
        ClassQuery!();
        CanonicalClassQuery!();
        Result!();
        Error!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < 'a > ClassQuery < 'a > { fn canonicalize (& self) -> Result < CanonicalClassQuery , Error > { match * self { ClassQuery :: OneLetter (c) => self . canonical_binary (& c . to_string ()) , ClassQuery :: Binary (name) => self . canonical_binary (name) , ClassQuery :: ByValue { property_name , property_value } => { let property_name = symbolic_name_normalize (property_name) ; let property_value = symbolic_name_normalize (property_value) ; let canon_name = match canonical_prop (& property_name) ? { None => return Err (Error :: PropertyNotFound) , Some (canon_name) => canon_name , } ; Ok (match canon_name { "General_Category" => { let canon = match canonical_gencat (& property_value) ? { None => return Err (Error :: PropertyValueNotFound) , Some (canon) => canon , } ; CanonicalClassQuery :: GeneralCategory (canon) } "Script" => { let canon = match canonical_script (& property_value) ? { None => return Err (Error :: PropertyValueNotFound) , Some (canon) => canon , } ; CanonicalClassQuery :: Script (canon) } _ => { let vals = match property_values (canon_name) ? { None => return Err (Error :: PropertyValueNotFound) , Some (vals) => vals , } ; let canon_val = match canonical_value (vals , & property_value) { None => { return Err (Error :: PropertyValueNotFound) } Some (canon_val) => canon_val , } ; CanonicalClassQuery :: ByValue { property_name : canon_name , property_value : canon_val , } } }) } } } fn canonical_binary (& self , name : & str ,) -> Result < CanonicalClassQuery , Error > { let norm = symbolic_name_normalize (name) ; if norm != "cf" && norm != "sc" && norm != "lc" { if let Some (canon) = canonical_prop (& norm) ? { return Ok (CanonicalClassQuery :: Binary (canon)) ; } } if let Some (canon) = canonical_gencat (& norm) ? { return Ok (CanonicalClassQuery :: GeneralCategory (canon)) ; } if let Some (canon) = canonical_script (& norm) ? { return Ok (CanonicalClassQuery :: Script (canon)) ; } Err (Error :: PropertyNotFound) } }
    };
}

impl_288!();