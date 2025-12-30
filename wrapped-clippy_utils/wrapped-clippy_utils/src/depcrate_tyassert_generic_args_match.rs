// Generated macro for assert_generic_args_match (function)
macro_rules! Depcrate_tyassert_generic_args_match {
() => {
// Module: crate::ty
// Provides: {"assert_generic_args_match"}
// Dependencies: {}
# [cfg (debug_assertions)] # [doc = " Asserts that the given arguments match the generic parameters of the given item."] fn assert_generic_args_match < 'tcx > (tcx : TyCtxt < 'tcx > , did : DefId , args : & [GenericArg < 'tcx >]) { use itertools :: Itertools ; let g = tcx . generics_of (did) ; let parent = g . parent . map (| did | tcx . generics_of (did)) ; let count = g . parent_count + g . own_params . len () ; let params = parent . map_or ([] . as_slice () , | p | p . own_params . as_slice ()) . iter () . chain (& g . own_params) . map (| x | & x . kind) ; assert ! (count == args . len () , "wrong number of arguments for `{did:?}`: expected `{count}`, found {}\n\
            note: the expected arguments are: `[{}]`\n\
            the given arguments are: `{args:#?}`" , args . len () , params . clone () . map (ty :: GenericParamDefKind :: descr) . format (", ") ,) ; if let Some ((idx , (param , arg))) = params . clone () . zip (args . iter () . map (| & x | x . kind ())) . enumerate () . find (| (_ , (param , arg)) | match (param , arg) { (ty :: GenericParamDefKind :: Lifetime , GenericArgKind :: Lifetime (_)) | (ty :: GenericParamDefKind :: Type { .. } , GenericArgKind :: Type (_)) | (ty :: GenericParamDefKind :: Const { .. } , GenericArgKind :: Const (_)) => false , (ty :: GenericParamDefKind :: Lifetime | ty :: GenericParamDefKind :: Type { .. } | ty :: GenericParamDefKind :: Const { .. } , _ ,) => true , }) { panic ! ("incorrect argument for `{did:?}` at index `{idx}`: expected a {}, found `{arg:?}`\n\
                note: the expected arguments are `[{}]`\n\
                the given arguments are `{args:#?}`" , param . descr () , params . clone () . map (ty :: GenericParamDefKind :: descr) . format (", ") ,) ; } }
};
}
