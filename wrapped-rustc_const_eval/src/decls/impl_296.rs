macro_rules! deps {
    () => {
        PlaceTy!();
        MPlaceTy!();
        Place!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > PlaceTy < 'tcx , Prov > { # [inline (always)] pub (super) fn place (& self) -> & Place < Prov > { & self . place } # [doc = " A place is either an mplace or some local."] # [doc = ""] # [doc = " Note that the return value can be different even for logically identical places!"] # [doc = " Specifically, if a local is stored in-memory, this may return `Local` or `MPlaceTy`"] # [doc = " depending on how the place was constructed. In other words, seeing `Local` here does *not*"] # [doc = " imply that this place does not point to memory. Every caller must therefore always handle"] # [doc = " both cases."] # [inline (always)] pub fn as_mplace_or_local (& self ,) -> Either < MPlaceTy < 'tcx , Prov > , (mir :: Local , Option < Size > , usize , TyAndLayout < 'tcx >) > { match self . place { Place :: Ptr (mplace) => Left (MPlaceTy { mplace , layout : self . layout }) , Place :: Local { local , offset , locals_addr } => { Right ((local , offset , locals_addr , self . layout)) } } } # [inline (always)] # [cfg_attr (debug_assertions , track_caller)] pub fn assert_mem_place (& self) -> MPlaceTy < 'tcx , Prov > { self . as_mplace_or_local () . left () . unwrap_or_else (| | { bug ! ("PlaceTy of type {} was a local when it was expected to be an MPlace" , self . layout . ty) }) } }
    };
}

impl_296!();