macro_rules! deps {
    () => {
        ShouldEmit!();
        GroupType!();
        Late!();
        Stage!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        # [allow (private_interfaces)] impl Stage for Late { type Id = HirId ; fn parsers () -> & 'static GroupType < Self > { & late :: ATTRIBUTE_PARSERS } fn emit_err < 'sess > (& self , tcx : & 'sess Session , diag : impl for < 'x > Diagnostic < 'x > ,) -> ErrorGuaranteed { tcx . dcx () . emit_err (diag) } fn should_emit (& self) -> ShouldEmit { ShouldEmit :: ErrorsAndLints } fn id_is_crate_root (id : Self :: Id) -> bool { id == CRATE_HIR_ID } }
    };
}

impl_270!()