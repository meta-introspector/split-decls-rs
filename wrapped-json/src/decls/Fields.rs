macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! Fields {
    () => {
        deps!();
        # [doc = " Fields of a braced struct syntax tree node with named fields."] # [doc = ""] # [doc = " The keys in the map are the field names."] pub type Fields = IndexMap < String , Type > ;
    };
}

Fields!();