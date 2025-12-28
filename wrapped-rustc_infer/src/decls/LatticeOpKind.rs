macro_rules! LatticeOpKind {
    () => {
        # [derive (Clone , Copy)] pub (crate) enum LatticeOpKind { Glb , Lub , }
    };
}

LatticeOpKind!()