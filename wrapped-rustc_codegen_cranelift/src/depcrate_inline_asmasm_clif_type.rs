// Generated macro for asm_clif_type (function)
macro_rules! Depcrate_inline_asmasm_clif_type {
() => {
// Module: crate::inline_asm
// Provides: {"asm_clif_type"}
// Dependencies: {}
fn asm_clif_type < 'tcx > (fx : & FunctionCx < '_ , '_ , 'tcx > , ty : Ty < 'tcx >) -> Option < types :: Type > { match ty . kind () { ty :: Adt (adt , args) if fx . tcx . is_lang_item (adt . did () , LangItem :: MaybeUninit) => { let fields = & adt . non_enum_variant () . fields ; let ty = fields [FieldIdx :: ONE] . ty (fx . tcx , args) ; let ty :: Adt (ty , args) = ty . kind () else { unreachable ! ("expected first field of `MaybeUninit` to be an ADT") } ; assert ! (ty . is_manually_drop () , "expected first field of `MaybeUninit` to be `ManuallyDrop`") ; let fields = & ty . non_enum_variant () . fields ; let ty = fields [FieldIdx :: ZERO] . ty (fx . tcx , args) ; fx . clif_type (ty) } _ => fx . clif_type (ty) , } }
};
}
