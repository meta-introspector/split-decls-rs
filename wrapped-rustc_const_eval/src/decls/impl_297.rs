macro_rules! deps {
    () => {
        Place!();
        InterpCx!();
        PlaceTy!();
        OffsetMode!();
        MemPlaceMeta!();
        OpTy!();
        Machine!();
        Projectable!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > Projectable < 'tcx , Prov > for PlaceTy < 'tcx , Prov > { # [inline (always)] fn layout (& self) -> TyAndLayout < 'tcx > { self . layout } # [inline] fn meta (& self) -> MemPlaceMeta < Prov > { match self . as_mplace_or_local () { Left (mplace) => mplace . meta () , Right (_) => { debug_assert ! (self . layout . is_sized () , "unsized locals should live in memory") ; MemPlaceMeta :: None } } } fn offset_with_meta < M : Machine < 'tcx , Provenance = Prov > > (& self , offset : Size , mode : OffsetMode , meta : MemPlaceMeta < Prov > , layout : TyAndLayout < 'tcx > , ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , Self > { interp_ok (match self . as_mplace_or_local () { Left (mplace) => mplace . offset_with_meta (offset , mode , meta , layout , ecx) ? . into () , Right ((local , old_offset , locals_addr , _)) => { debug_assert ! (layout . is_sized () , "unsized locals should live in memory") ; assert_matches ! (meta , MemPlaceMeta :: None) ; assert ! (offset + layout . size <= self . layout . size) ; let new_offset = old_offset . unwrap_or (Size :: ZERO) + offset ; PlaceTy { place : Place :: Local { local , offset : Some (new_offset) , locals_addr } , layout , } } }) } # [inline (always)] fn to_op < M : Machine < 'tcx , Provenance = Prov > > (& self , ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , OpTy < 'tcx , M :: Provenance > > { ecx . place_to_op (self) } }
    };
}

impl_297!()