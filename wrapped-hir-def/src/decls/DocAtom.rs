macro_rules! DocAtom {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum DocAtom { # [doc = " eg. `#[doc(hidden)]`"] Flag (Symbol) , # [doc = " eg. `#[doc(alias = \"it\")]`"] # [doc = ""] # [doc = " Note that a key can have multiple values that are all considered \"active\" at the same time."] # [doc = " For example, `#[doc(alias = \"x\")]` and `#[doc(alias = \"y\")]`."] KeyValue { key : Symbol , value : Symbol } , }
    };
}

DocAtom!();