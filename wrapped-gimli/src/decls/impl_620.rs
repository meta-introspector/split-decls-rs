macro_rules! deps {
    () => {
        ReaderOffset!();
        UnitType!();
    };
}

macro_rules! impl_620 {
    () => {
        deps!();
        impl < Offset > UnitType < Offset > where Offset : ReaderOffset , { # [allow (unused)] pub (crate) fn dw_ut (& self) -> constants :: DwUt { match self { UnitType :: Compilation => constants :: DW_UT_compile , UnitType :: Type { .. } => constants :: DW_UT_type , UnitType :: Partial => constants :: DW_UT_partial , UnitType :: Skeleton (_) => constants :: DW_UT_skeleton , UnitType :: SplitCompilation (_) => constants :: DW_UT_split_compile , UnitType :: SplitType { .. } => constants :: DW_UT_split_type , } } }
    };
}

impl_620!()