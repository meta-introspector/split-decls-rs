macro_rules! deps {
    () => {
        OpTy!();
        Projectable!();
        OffsetMode!();
        Machine!();
        InterpCx!();
        MemPlaceMeta!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > Projectable < 'tcx , Prov > for OpTy < 'tcx , Prov > { # [inline (always)] fn layout (& self) -> TyAndLayout < 'tcx > { self . layout } # [inline] fn meta (& self) -> MemPlaceMeta < Prov > { match self . as_mplace_or_imm () { Left (mplace) => mplace . meta () , Right (_) => { debug_assert ! (self . layout . is_sized () , "unsized immediates are not a thing") ; MemPlaceMeta :: None } } } fn offset_with_meta < M : Machine < 'tcx , Provenance = Prov > > (& self , offset : Size , mode : OffsetMode , meta : MemPlaceMeta < Prov > , layout : TyAndLayout < 'tcx > , ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , Self > { match self . as_mplace_or_imm () { Left (mplace) => { interp_ok (mplace . offset_with_meta (offset , mode , meta , layout , ecx) ? . into ()) } Right (imm) => { assert_matches ! (meta , MemPlaceMeta :: None) ; interp_ok (imm . offset_ (offset , layout , ecx) . into ()) } } } # [inline (always)] fn to_op < M : Machine < 'tcx , Provenance = Prov > > (& self , _ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , OpTy < 'tcx , M :: Provenance > > { interp_ok (self . clone ()) } }
    };
}

impl_278!()