// Generated macro for extract_first_ty_param (function)
macro_rules! Depcrate_parserextract_first_ty_param {
() => {
// Module: crate::parser
// Provides: {"extract_first_ty_param"}
// Dependencies: {}
# [doc = " Get the first type parameter of a generic type, errors on incorrect input."] fn extract_first_ty_param (ty : Option < & syn :: Type >) -> Result < Option < syn :: Type > , Diagnostic > { let t = match ty { Some (t) => t , None => return Ok (None) , } ; let path = match * get_ty (t) { syn :: Type :: Path (syn :: TypePath { qself : None , ref path , }) => path , _ => bail_span ! (t , "must be Result<...>") , } ; let seg = path . segments . last () . ok_or_else (| | err_span ! (t , "must have at least one segment")) ? ; let generics = match seg . arguments { syn :: PathArguments :: AngleBracketed (ref t) => t , _ => bail_span ! (t , "must be Result<...>") , } ; let generic = generics . args . first () . ok_or_else (| | err_span ! (t , "must have at least one generic parameter")) ? ; let ty = match generic { syn :: GenericArgument :: Type (t) => t , other => bail_span ! (other , "must be a type parameter") , } ; match get_ty (ty) { syn :: Type :: Tuple (t) if t . elems . is_empty () => return Ok (None) , _ => { } } Ok (Some (ty . clone ())) }
};
}
