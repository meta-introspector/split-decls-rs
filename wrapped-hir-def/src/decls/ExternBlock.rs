macro_rules! ExternBlock {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct ExternBlock { pub (crate) children : Box < [ModItemId] > , }
    };
}

ExternBlock!()