macro_rules! deps {
    () => {
        Identifier!();
        Id!();
    };
}

macro_rules! PendingArg {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct PendingArg { pub (crate) id : Id , pub (crate) ident : Option < Identifier > , pub (crate) raw_vals : Vec < OsString > , pub (crate) trailing_idx : Option < usize > , }
    };
}

PendingArg!()