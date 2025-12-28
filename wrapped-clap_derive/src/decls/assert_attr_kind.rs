macro_rules! deps {
    () => {
        ClapAttr!();
        AttrKind!();
    };
}

macro_rules! assert_attr_kind {
    () => {
        deps!();
        fn assert_attr_kind (attr : & ClapAttr , possible_kind : & [AttrKind]) -> Result < () , syn :: Error > { if * attr . kind . get () == AttrKind :: Clap || * attr . kind . get () == AttrKind :: StructOpt { } else if ! possible_kind . contains (attr . kind . get ()) { let options = possible_kind . iter () . map (| k | format ! ("`#[{}({})]`" , k . as_str () , attr . name)) . collect :: < Vec < _ > > () ; abort ! (attr . name , "Unknown `#[{}({})]` attribute ({} exists)" , attr . kind . as_str () , attr . name , options . join (", ")) ; } Ok (()) }
    };
}

assert_attr_kind!();