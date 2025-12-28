macro_rules! deps {
    () => {
        UndoLog!();
    };
}

macro_rules! impl_from {
    () => {
        deps!();
        macro_rules ! impl_from { ($ ($ ctor : ident ($ ty : ty) ,) *) => { $ (impl <'tcx > From <$ ty > for UndoLog <'tcx > { fn from (x : $ ty) -> Self { UndoLog ::$ ctor (x . into ()) } }) * } }
    };
}

impl_from!();