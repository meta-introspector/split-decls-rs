// Generated macro for _impls (module)
macro_rules! Depcrate_spec_impls {
() => {
// Module: crate::spec
// Provides: {"_impls"}
// Dependencies: {}
mod _impls { use std :: fmt :: { Display , Formatter } ; use crate :: Spec ; impl Display for Spec { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { match self { Spec :: Include (oid) => Display :: fmt (oid , f) , Spec :: Exclude (oid) => write ! (f , "^{oid}") , Spec :: Range { from , to } => write ! (f , "{from}..{to}") , Spec :: Merge { theirs , ours } => write ! (f , "{theirs}...{ours}") , Spec :: IncludeOnlyParents (from_exclusive) => write ! (f , "{from_exclusive}^@") , Spec :: ExcludeParents (oid) => write ! (f , "{oid}^!") , } } } }
};
}
