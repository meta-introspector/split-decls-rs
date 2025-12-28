macro_rules! deps {
    () => {
        MPlaceTy!();
        Projectable!();
        MemPlaceMeta!();
        InterpCx!();
        OffsetMode!();
        Machine!();
        OpTy!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > Projectable < 'tcx , Prov > for MPlaceTy < 'tcx , Prov > { # [inline (always)] fn layout (& self) -> TyAndLayout < 'tcx > { self . layout } # [inline (always)] fn meta (& self) -> MemPlaceMeta < Prov > { self . mplace . meta } fn offset_with_meta < M : Machine < 'tcx , Provenance = Prov > > (& self , offset : Size , mode : OffsetMode , meta : MemPlaceMeta < Prov > , layout : TyAndLayout < 'tcx > , ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , Self > { interp_ok (MPlaceTy { mplace : self . mplace . offset_with_meta_ (offset , mode , meta , ecx) ? , layout , }) } # [inline (always)] fn to_op < M : Machine < 'tcx , Provenance = Prov > > (& self , _ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , OpTy < 'tcx , M :: Provenance > > { interp_ok (self . clone () . into ()) } }
    };
}

impl_291!()