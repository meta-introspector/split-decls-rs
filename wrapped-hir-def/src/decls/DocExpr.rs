macro_rules! deps {
    () => {
        DocAtom!();
    };
}

macro_rules! DocExpr {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum DocExpr { Invalid , # [doc = " eg. `#[doc(hidden)]`, `#[doc(alias = \"x\")]`"] Atom (DocAtom) , # [doc = " eg. `#[doc(alias(\"x\", \"y\"))]`"] Alias (Vec < Symbol >) , }
    };
}

DocExpr!()