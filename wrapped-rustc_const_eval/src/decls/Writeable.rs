macro_rules! deps {
    () => {
        Machine!();
        MPlaceTy!();
        Projectable!();
        PlaceTy!();
        InterpCx!();
    };
}

macro_rules! Writeable {
    () => {
        deps!();
        # [doc = " The `Weiteable` trait describes interpreter values that can be written to."] pub trait Writeable < 'tcx , Prov : Provenance > : Projectable < 'tcx , Prov > { fn to_place (& self) -> PlaceTy < 'tcx , Prov > ; fn force_mplace < M : Machine < 'tcx , Provenance = Prov > > (& self , ecx : & mut InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , MPlaceTy < 'tcx , Prov > > ; }
    };
}

Writeable!()