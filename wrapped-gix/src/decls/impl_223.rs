macro_rules! deps {
    () => {
        Tree!();
        BreadthFirstPresets!();
        Platform!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        # [doc = " Traversal"] impl < 'repo > Tree < 'repo > { # [doc = " Obtain a platform for initiating a variety of traversals."] pub fn traverse (& self) -> Platform < '_ , 'repo > { Platform { root : self , breadthfirst : BreadthFirstPresets { root : self } , } } }
    };
}

impl_223!();