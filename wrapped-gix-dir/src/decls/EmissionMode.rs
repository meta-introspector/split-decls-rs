macro_rules! deps {
    () => {
        Delegate!();
    };
}

macro_rules! EmissionMode {
    () => {
        deps!();
        # [doc = " The way entries are emitted using the [Delegate]."] # [doc = ""] # [doc = " The choice here controls if entries are emitted immediately, or have to be held back."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum EmissionMode { # [doc = " Emit each entry as it matches exactly, without doing any kind of simplification."] # [doc = ""] # [doc = " Emissions in this mode are happening as they occur, without any buffering or ordering."] # [default] Matching , # [doc = " Emit only a containing directory if all of its entries are of the same type."] # [doc = ""] # [doc = " Note that doing so is more expensive as it requires us to keep track of all entries in the directory structure"] # [doc = " until it's clear what to finally emit."] CollapseDirectory , }
    };
}

EmissionMode!()