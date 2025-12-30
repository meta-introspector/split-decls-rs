// Generated macro for normalize_meta (function)
macro_rules! Depcrate_attrnormalize_meta {
() => {
// Module: crate::attr
// Provides: {"normalize_meta"}
// Dependencies: {}
# [doc = " Normalize a `meta: Meta` into the forms accepted in `#[proptest(<meta>)]`."] fn normalize_meta (meta : Meta) -> Option < NormMeta > { match meta { Meta :: Path (_) => Some (NormMeta :: Plain) , Meta :: NameValue (nv) => match nv . value { Expr :: Lit (elit) => Some (NormMeta :: Lit (elit . lit)) , _ => None , } Meta :: List (ml) => { let mut output : Option < NormMeta > = None ; if let Ok (lit) = syn :: parse2 (ml . tokens . clone ()) { output = Some (NormMeta :: Lit (lit)) ; } else if let Ok (ident) = syn :: parse2 (ml . tokens . clone ()) { output = Some (NormMeta :: Word (ident)) ; } output } } }
};
}
