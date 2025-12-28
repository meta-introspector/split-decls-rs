macro_rules! deps {
    () => {
        FieldAttributes!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl FromAttribute for FieldAttributes { fn parse (group : & Group) -> Result < Option < Self > > { let attributes = match parse_tagged_attribute (group , "bincode") ? { Some (body) => body , None => return Ok (None) , } ; let mut result = Self :: default () ; for attribute in attributes { match attribute { ParsedAttribute :: Tag (i) if i . to_string () == "with_serde" => { result . with_serde = true ; } ParsedAttribute :: Tag (i) => { return Err (Error :: custom_at ("Unknown field attribute" , i . span ())) } ParsedAttribute :: Property (key , _) => { return Err (Error :: custom_at ("Unknown field attribute" , key . span ())) } _ => { } } } Ok (Some (result)) } }
    };
}

impl_4!();