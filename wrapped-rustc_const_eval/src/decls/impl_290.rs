macro_rules! deps {
    () => {
        MemPlace!();
        MPlaceTy!();
        Immediate!();
        MemPlaceMeta!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > MPlaceTy < 'tcx , Prov > { # [doc = " Produces a MemPlace that works for ZST but nothing else."] # [doc = " Conceptually this is a new allocation, but it doesn't actually create an allocation so you"] # [doc = " don't need to worry about memory leaks."] # [inline] pub fn fake_alloc_zst (layout : TyAndLayout < 'tcx >) -> Self { assert ! (layout . is_zst ()) ; let align = layout . align . abi ; let ptr = Pointer :: without_provenance (align . bytes ()) ; MPlaceTy { mplace : MemPlace { ptr , meta : MemPlaceMeta :: None , misaligned : None } , layout } } # [doc = " Adjust the provenance of the main pointer (metadata is unaffected)."] pub fn map_provenance (self , f : impl FnOnce (Prov) -> Prov) -> Self { MPlaceTy { mplace : self . mplace . map_provenance (f) , .. self } } # [inline (always)] pub (super) fn mplace (& self) -> & MemPlace < Prov > { & self . mplace } # [inline (always)] pub fn ptr (& self) -> Pointer < Option < Prov > > { self . mplace . ptr } # [inline (always)] pub fn to_ref (& self , cx : & impl HasDataLayout) -> Immediate < Prov > { self . mplace . to_ref (cx) } }
    };
}

impl_290!()