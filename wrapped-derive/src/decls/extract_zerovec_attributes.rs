macro_rules! extract_zerovec_attributes {
    () => {
        # [doc = " Removes all attributes with `zerovec` in the name and places them in a separate vector"] pub fn extract_zerovec_attributes (attrs : & mut Vec < Attribute >) -> Vec < Attribute > { let mut ret = vec ! [] ; attrs . retain (| a | { if a . path () . segments . len () == 2 && a . path () . segments [0] . ident == "zerovec" { ret . push (a . clone ()) ; return false ; } true }) ; ret }
    };
}

extract_zerovec_attributes!()