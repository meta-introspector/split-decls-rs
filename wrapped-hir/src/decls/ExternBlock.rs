macro_rules! ExternBlock {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct ExternBlock { pub (crate) id : ExternBlockId , }
    };
}

ExternBlock!();