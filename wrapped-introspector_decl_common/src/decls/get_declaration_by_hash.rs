macro_rules! deps {
    () => {
        DeclInfo!();
    };
}

macro_rules! get_declaration_by_hash {
    () => {
        deps!();
        pub fn get_declaration_by_hash (hash : & str) -> Option < DeclInfo > { DECL_REGISTRY . lock () . ok () ? . by_hash . get (hash) . and_then (| & idx | { DECL_REGISTRY . lock () . ok () ? . declarations . get (idx) . cloned () }) }
    };
}

get_declaration_by_hash!();