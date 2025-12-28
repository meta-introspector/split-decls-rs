macro_rules! identifiers {
    () => {
        macro_rules ! identifiers { ($ ($ name : ident) +) => { $ (# [derive (Default , Copy , Clone , Hash , PartialEq , Eq , Ord , PartialOrd , Debug)] pub struct $ name (pub (crate) u32) ;) * } ; }
    };
}

identifiers!();