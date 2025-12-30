// Generated macro for validate_impl (function)
macro_rules! Depcrate_pinned_dropvalidate_impl {
() => {
// Module: crate::pinned_drop
// Provides: {"validate_impl"}
// Dependencies: {}
# [doc = " Validates the signature of given `PinnedDrop` impl."] fn validate_impl (item : & ItemImpl) -> Result < () > { const INVALID_ITEM : & str = "#[pinned_drop] may only be used on implementation for the `PinnedDrop` trait" ; if let Some (attr) = item . attrs . find ("pinned_drop") { bail ! (attr , "duplicate #[pinned_drop] attribute") ; } if let Some ((_ , path , _)) = & item . trait_ { if ! path . is_ident ("PinnedDrop") { bail ! (path , INVALID_ITEM) ; } } else { bail ! (item . self_ty , INVALID_ITEM) ; } if item . unsafety . is_some () { bail ! (item . unsafety , "implementing the trait `PinnedDrop` is not unsafe") ; } if item . items . is_empty () { bail ! (item , "not all trait items implemented, missing: `drop`") ; } match & * item . self_ty { Type :: Path (_) => { } ty => { bail ! (ty , "implementing the trait `PinnedDrop` on this type is unsupported") ; } } item . items . iter () . enumerate () . try_for_each (| (i , item) | match item { ImplItem :: Const (item) => { bail ! (item , "const `{}` is not a member of trait `PinnedDrop`" , item . ident) } ImplItem :: Type (item) => { bail ! (item , "type `{}` is not a member of trait `PinnedDrop`" , item . ident) } ImplItem :: Fn (method) => { validate_sig (& method . sig) ? ; if i == 0 { Ok (()) } else { bail ! (method , "duplicate definitions with name `drop`") } } _ => unreachable ! ("unexpected ImplItem") , }) }
};
}
