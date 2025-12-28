macro_rules! ExternCrateDecl {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct ExternCrateDecl { pub (crate) id : ExternCrateId , }
    };
}

ExternCrateDecl!();