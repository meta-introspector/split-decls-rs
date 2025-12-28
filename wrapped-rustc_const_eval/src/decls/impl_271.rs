macro_rules! deps {
    () => {
        OpTy!();
        Machine!();
        MemPlaceMeta!();
        Projectable!();
        InterpCx!();
        ImmTy!();
        OffsetMode!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > Projectable < 'tcx , Prov > for ImmTy < 'tcx , Prov > { # [inline (always)] fn layout (& self) -> TyAndLayout < 'tcx > { self . layout } # [inline (always)] fn meta (& self) -> MemPlaceMeta < Prov > { debug_assert ! (self . layout . is_sized ()) ; MemPlaceMeta :: None } fn offset_with_meta < M : Machine < 'tcx , Provenance = Prov > > (& self , offset : Size , _mode : OffsetMode , meta : MemPlaceMeta < Prov > , layout : TyAndLayout < 'tcx > , ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , Self > { assert_matches ! (meta , MemPlaceMeta :: None) ; interp_ok (self . offset_ (offset , layout , ecx)) } # [inline (always)] fn to_op < M : Machine < 'tcx , Provenance = Prov > > (& self , _ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , OpTy < 'tcx , M :: Provenance > > { interp_ok (self . clone () . into ()) } }
    };
}

impl_271!();