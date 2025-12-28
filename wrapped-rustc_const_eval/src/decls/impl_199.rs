macro_rules! deps {
    () => {
        FnArg!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > FnArg < 'tcx , Prov > { pub fn layout (& self) -> & TyAndLayout < 'tcx > { match self { FnArg :: Copy (op) => & op . layout , FnArg :: InPlace (mplace) => & mplace . layout , } } }
    };
}

impl_199!()