macro_rules! deps {
    () => {
        FeatureMap!();
    };
}

macro_rules! create_index_line {
    () => {
        deps!();
        pub (crate) fn create_index_line (name : serde_json :: Value , vers : & str , deps : Vec < serde_json :: Value > , cksum : & str , features : crate :: registry :: FeatureMap , yanked : bool , links : Option < String > , rust_version : Option < & str > , pubtime : Option < & str > , v : Option < u32 > ,) -> String { let (features , features2) = split_index_features (features . clone ()) ; let mut json = serde_json :: json ! ({ "name" : name , "vers" : vers , "deps" : deps , "cksum" : cksum , "features" : features , "yanked" : yanked , "links" : links , }) ; if let Some (f2) = & features2 { json ["features2"] = serde_json :: json ! (f2) ; json ["v"] = serde_json :: json ! (2) ; } if let Some (v) = v { json ["v"] = serde_json :: json ! (v) ; } if let Some (rust_version) = rust_version { json ["rust_version"] = serde_json :: json ! (rust_version) ; } if let Some (pubtime) = pubtime { json ["pubtime"] = serde_json :: json ! (pubtime) ; } json . to_string () }
    };
}

create_index_line!();