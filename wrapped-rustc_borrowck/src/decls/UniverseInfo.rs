macro_rules! deps {
    () => {
        TypeOpInfo!();
    };
}

macro_rules! UniverseInfo {
    () => {
        deps!();
        # [doc = " What operation a universe was created for."] # [derive (Clone)] pub (crate) enum UniverseInfo < 'tcx > { # [doc = " Relating two types which have binders."] RelateTys { expected : Ty < 'tcx > , found : Ty < 'tcx > } , # [doc = " Created from performing a `TypeOp`."] TypeOp (Rc < dyn TypeOpInfo < 'tcx > + 'tcx >) , # [doc = " Any other reason."] Other , }
    };
}

UniverseInfo!();