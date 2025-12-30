// Generated macro for is_ty_alias (function)
macro_rules! Depcrateis_ty_alias {
() => {
// Module: crate
// Provides: {"is_ty_alias"}
// Dependencies: {}
# [doc = " Checks if the given `QPath` belongs to a type alias."] pub fn is_ty_alias (qpath : & QPath < '_ >) -> bool { match * qpath { QPath :: Resolved (_ , path) => matches ! (path . res , Res :: Def (DefKind :: TyAlias | DefKind :: AssocTy , ..)) , QPath :: TypeRelative (ty , _) if let TyKind :: Path (qpath) = ty . kind => is_ty_alias (& qpath) , QPath :: TypeRelative (..) => false , } }
};
}
