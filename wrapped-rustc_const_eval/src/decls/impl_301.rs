macro_rules! deps {
    () => {
        Writeable!();
        MPlaceTy!();
        Machine!();
        PlaceTy!();
        InterpCx!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > Writeable < 'tcx , Prov > for MPlaceTy < 'tcx , Prov > { # [inline (always)] fn to_place (& self) -> PlaceTy < 'tcx , Prov > { self . clone () . into () } # [inline (always)] fn force_mplace < M : Machine < 'tcx , Provenance = Prov > > (& self , _ecx : & mut InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , MPlaceTy < 'tcx , Prov > > { interp_ok (self . clone ()) } }
    };
}

impl_301!();