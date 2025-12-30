// Generated macro for impl_199 (impl)
macro_rules! Depcrate_parse_futureimpl_199 {
() => {
// Module: crate::parse::future
// Provides: {"impl_199"}
// Dependencies: {}
impl FutureBuilder { fn compute_arguments_kind (arg : & syn :: Attribute) -> syn :: Result < FutureArg > { if matches ! (arg . meta , syn :: Meta :: Path (_)) { Ok (FutureArg :: Define) } else { match arg . parse_args :: < Option < Ident > > () ? { Some (awt) if awt == format_ident ! ("awt") => Ok (FutureArg :: Await) , None => Ok (FutureArg :: Define) , Some (invalid) => Err (syn :: Error :: new_spanned (arg . parse_args :: < Option < Ident > > () ? . into_token_stream () , format ! ("Invalid '{invalid}' #[future(...)] arg.") ,)) , } } } }
};
}
