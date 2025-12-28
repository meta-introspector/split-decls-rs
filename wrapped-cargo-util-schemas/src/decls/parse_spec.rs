macro_rules! deps {
    () => {
        PartialVersion!();
        Result!();
    };
}

macro_rules! parse_spec {
    () => {
        deps!();
        fn parse_spec (spec : & str) -> Result < Option < (String , Option < PartialVersion >) > > { let Some ((name , ver)) = spec . rsplit_once ('@') . or_else (| | spec . rsplit_once (':') . filter (| (n , _) | ! n . ends_with (':'))) else { return Ok (None) ; } ; let name = name . to_owned () ; let ver = ver . parse :: < PartialVersion > () ? ; Ok (Some ((name , Some (ver)))) }
    };
}

parse_spec!();