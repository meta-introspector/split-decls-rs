macro_rules! deps {
    () => {
        MPlaceTy!();
        Place!();
        PlaceTy!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > From < MPlaceTy < 'tcx , Prov > > for PlaceTy < 'tcx , Prov > { # [inline (always)] fn from (mplace : MPlaceTy < 'tcx , Prov >) -> Self { PlaceTy { place : Place :: Ptr (mplace . mplace) , layout : mplace . layout } } }
    };
}

impl_295!()