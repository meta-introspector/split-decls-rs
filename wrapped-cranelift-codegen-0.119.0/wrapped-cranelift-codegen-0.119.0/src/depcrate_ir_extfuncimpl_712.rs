// Generated macro for impl_712 (impl)
macro_rules! Depcrate_ir_extfuncimpl_712 {
() => {
// Module: crate::ir::extfunc
// Provides: {"impl_712"}
// Dependencies: {}
impl FromStr for ArgumentPurpose { type Err = () ; fn from_str (s : & str) -> Result < Self , () > { match s { "normal" => Ok (Self :: Normal) , "sret" => Ok (Self :: StructReturn) , "vmctx" => Ok (Self :: VMContext) , _ if s . starts_with ("sarg(") => { if ! s . ends_with (")") { return Err (()) ; } let size : u32 = s ["sarg(" . len () .. s . len () - 1] . parse () . map_err (| _ | ()) ? ; Ok (Self :: StructArgument (size)) } _ => Err (()) , } } }
};
}
