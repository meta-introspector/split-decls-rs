macro_rules! deps {
    () => {
        Writeable!();
        InterpCx!();
        PlaceTy!();
        MPlaceTy!();
        Machine!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > Writeable < 'tcx , Prov > for PlaceTy < 'tcx , Prov > { # [inline (always)] fn to_place (& self) -> PlaceTy < 'tcx , Prov > { self . clone () } # [inline (always)] fn force_mplace < M : Machine < 'tcx , Provenance = Prov > > (& self , ecx : & mut InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , MPlaceTy < 'tcx , Prov > > { ecx . force_allocation (self) } }
    };
}

impl_300!();