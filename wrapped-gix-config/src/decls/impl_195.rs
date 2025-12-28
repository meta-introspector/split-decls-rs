macro_rules! deps {
    () => {
        KeyRef!();
        Key!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl KeyRef < '_ > { # [doc = " Parse `input` like `core.bare` or `remote.origin.url` as a `Key` to make its fields available,"] # [doc = " or `None` if there were not at least 2 tokens separated by `.`."] # [doc = " Note that `input` isn't validated, and is `str` as ascii is a subset of UTF-8 which is required for any valid keys."] pub fn parse_unvalidated (input : & BStr) -> Option < KeyRef < '_ > > { let mut tokens = input . splitn (2 , | b | * b == b'.') ; let section_name = tokens . next () ? ; let subsection_or_key = tokens . next () ? ; let mut tokens = subsection_or_key . rsplitn (2 , | b | * b == b'.') ; let (subsection_name , value_name) = match (tokens . next () , tokens . next ()) { (Some (key) , Some (subsection)) => (Some (subsection . into ()) , key) , (Some (key) , None) => (None , key) , (None , Some (_)) => unreachable ! ("iterator can't restart producing items") , (None , None) => return None , } ; Some (KeyRef { section_name : section_name . to_str () . ok () ? , subsection_name , value_name : value_name . to_str () . ok () ? , }) } }
    };
}

impl_195!();