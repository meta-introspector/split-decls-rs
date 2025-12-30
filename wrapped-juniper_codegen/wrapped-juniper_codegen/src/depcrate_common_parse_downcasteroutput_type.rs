// Generated macro for output_type (function)
macro_rules! Depcrate_common_parse_downcasteroutput_type {
() => {
// Module: crate::common::parse::downcaster
// Provides: {"output_type"}
// Dependencies: {}
# [doc = " Parses downcasting output type from the downcaster method return type."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If return type is invalid (not `Option<&OutputType>`), then returns the [`Span`] to display the"] # [doc = " corresponding error at."] pub (crate) fn output_type (ret_ty : & syn :: ReturnType) -> Result < syn :: Type , Span > { let ret_ty = match & ret_ty { syn :: ReturnType :: Type (_ , ty) => & * * ty , _ => return Err (ret_ty . span ()) , } ; let path = match ret_ty . unparenthesized () { syn :: Type :: Path (syn :: TypePath { qself : None , path }) => path , _ => return Err (ret_ty . span ()) , } ; let (ident , args) = match path . segments . last () { Some (syn :: PathSegment { ident , arguments : syn :: PathArguments :: AngleBracketed (generic) , }) => (ident , & generic . args) , _ => return Err (ret_ty . span ()) , } ; if ident . unraw () != "Option" { return Err (ret_ty . span ()) ; } if args . len () != 1 { return Err (ret_ty . span ()) ; } let out_ty = match args . first () { Some (syn :: GenericArgument :: Type (inner_ty)) => match inner_ty . unparenthesized () { syn :: Type :: Reference (inner_ty) => { if inner_ty . mutability . is_some () { return Err (inner_ty . span ()) ; } inner_ty . elem . unparenthesized () . clone () } _ => return Err (ret_ty . span ()) , } , _ => return Err (ret_ty . span ()) , } ; Ok (out_ty) }
};
}
