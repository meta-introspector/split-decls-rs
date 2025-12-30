// Generated macro for needs_ordered_drop (function)
macro_rules! Depcrate_tyneeds_ordered_drop {
() => {
// Module: crate::ty
// Provides: {"needs_ordered_drop"}
// Dependencies: {}
# [doc = " Checks if the drop order for a type matters."] # [doc = ""] # [doc = " Some std types implement drop solely to deallocate memory. For these types, and composites"] # [doc = " containing them, changing the drop order won't result in any observable side effects."] pub fn needs_ordered_drop < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { fn needs_ordered_drop_inner < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , seen : & mut FxHashSet < Ty < 'tcx > >) -> bool { if ! seen . insert (ty) { return false ; } if ! ty . has_significant_drop (cx . tcx , cx . typing_env ()) { false } else if ty . is_lang_item (cx , LangItem :: OwnedBox) || matches ! (ty . opt_diag_name (cx) , Some (sym :: HashSet | sym :: Rc | sym :: Arc | sym :: cstring_type | sym :: RcWeak | sym :: ArcWeak)) { if let ty :: Adt (_ , subs) = ty . kind () { subs . types () . any (| ty | needs_ordered_drop_inner (cx , ty , seen)) } else { true } } else if ! cx . tcx . lang_items () . drop_trait () . is_some_and (| id | implements_trait (cx , ty , id , & [])) { match ty . kind () { ty :: Tuple (fields) => fields . iter () . any (| ty | needs_ordered_drop_inner (cx , ty , seen)) , ty :: Array (ty , _) => needs_ordered_drop_inner (cx , * ty , seen) , ty :: Adt (adt , subs) => adt . all_fields () . map (| f | f . ty (cx . tcx , subs)) . any (| ty | needs_ordered_drop_inner (cx , ty , seen)) , _ => true , } } else { true } } needs_ordered_drop_inner (cx , ty , & mut FxHashSet :: default ()) }
};
}
