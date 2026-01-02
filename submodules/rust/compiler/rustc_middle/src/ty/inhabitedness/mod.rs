mkuse!{use rustc_type_ir :: TyKind :: * ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: query :: Providers ;}
mkuse!{use crate :: ty :: context :: TyCtxt ;}
mkuse!{use crate :: ty :: { self , DefId , Ty , TypeVisitableExt , VariantDef , Visibility } ;}
mkmod!{inhabited_predicate, { 
                getname!(inhabited_predicate);
                getsrc!(inhabited_predicate);
                getpath!(inhabited_predicate);
                get_deps!(inhabited_predicate);
                get_crates!(inhabited_predicate);
                mkinclude!(inhabited_predicate);
                 
            }}
mkuse!{pub use inhabited_predicate :: InhabitedPredicate ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { inhabited_predicate_adt , inhabited_predicate_type , .. * providers } ; }
}

macro_rules! inhabited_predicate_adt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inhabited_predicate_adt in module {}", module_path!());
    };
}

mkfn!{
    inhabited_predicate_adt_introspect!();
    # [doc = " Returns an `InhabitedPredicate` that is generic over type parameters and"] # [doc = " requires calling [`InhabitedPredicate::instantiate`]"] fn inhabited_predicate_adt (tcx : TyCtxt < '_ > , def_id : DefId) -> InhabitedPredicate < '_ > { if let Some (def_id) = def_id . as_local () { if matches ! (tcx . representability (def_id) , ty :: Representability :: Infinite (_)) { return InhabitedPredicate :: True ; } } let adt = tcx . adt_def (def_id) ; InhabitedPredicate :: any (tcx , adt . variants () . iter () . map (| variant | variant . inhabited_predicate (tcx , adt)) ,) }
}
mkitem!{mkimpl!{impl < 'tcx > VariantDef { # [doc = " Calculates the forest of `DefId`s from which this variant is visibly uninhabited."] pub fn inhabited_predicate (& self , tcx : TyCtxt < 'tcx > , adt : ty :: AdtDef < '_ > ,) -> InhabitedPredicate < 'tcx > { debug_assert ! (! adt . is_union ()) ; InhabitedPredicate :: all (tcx , self . fields . iter () . map (| field | { let pred = tcx . type_of (field . did) . instantiate_identity () . inhabited_predicate (tcx) ; if adt . is_enum () { return pred ; } match field . vis { Visibility :: Public => pred , Visibility :: Restricted (from) => { pred . or (tcx , InhabitedPredicate :: NotInModule (from)) } } }) ,) } }}}
mkitem!{mkimpl!{impl < 'tcx > Ty < 'tcx > { # [instrument (level = "debug" , skip (tcx) , ret)] pub fn inhabited_predicate (self , tcx : TyCtxt < 'tcx >) -> InhabitedPredicate < 'tcx > { debug_assert ! (! self . has_infer ()) ; match self . kind () { Adt (adt , _) if adt . is_union () => InhabitedPredicate :: True , Adt (adt , _) if adt . variant_list_has_applicable_non_exhaustive () => { InhabitedPredicate :: True } Never => InhabitedPredicate :: False , Param (_) | Alias (ty :: Inherent | ty :: Projection | ty :: Free , _) => { InhabitedPredicate :: GenericType (self) } Alias (ty :: Opaque , alias_ty) => { match alias_ty . def_id . as_local () { None => InhabitedPredicate :: True , Some (local_def_id) => { let key = ty :: OpaqueTypeKey { def_id : local_def_id , args : alias_ty . args } ; InhabitedPredicate :: OpaqueType (key) } } } Tuple (tys) if tys . is_empty () => InhabitedPredicate :: True , Adt (..) | Array (..) | Tuple (_) => tcx . inhabited_predicate_type (self) , _ => InhabitedPredicate :: True , } } # [doc = " Checks whether a type is visibly uninhabited from a particular module."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " #![feature(never_type)]"] # [doc = " # fn main() {}"] # [doc = " enum Void {}"] # [doc = " mod a {"] # [doc = "     pub mod b {"] # [doc = "         pub struct SecretlyUninhabited {"] # [doc = "             _priv: !,"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " mod c {"] # [doc = "     use super::Void;"] # [doc = "     pub struct AlsoSecretlyUninhabited {"] # [doc = "         _priv: Void,"] # [doc = "     }"] # [doc = "     mod d {"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " struct Foo {"] # [doc = "     x: a::b::SecretlyUninhabited,"] # [doc = "     y: c::AlsoSecretlyUninhabited,"] # [doc = " }"] # [doc = " ```"] # [doc = " In this code, the type `Foo` will only be visibly uninhabited inside the"] # [doc = " modules b, c and d. This effects pattern-matching on `Foo` or types that"] # [doc = " contain `Foo`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```ignore (illustrative)"] # [doc = " let foo_result: Result<T, Foo> = ... ;"] # [doc = " let Ok(t) = foo_result;"] # [doc = " ```"] # [doc = " This code should only compile in modules where the uninhabitedness of Foo is"] # [doc = " visible."] pub fn is_inhabited_from (self , tcx : TyCtxt < 'tcx > , module : DefId , typing_env : ty :: TypingEnv < 'tcx > ,) -> bool { self . inhabited_predicate (tcx) . apply (tcx , typing_env , module) } # [doc = " Returns true if the type is uninhabited without regard to visibility"] pub fn is_privately_uninhabited (self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> bool { ! self . inhabited_predicate (tcx) . apply_ignore_module (tcx , typing_env) } }}}

macro_rules! inhabited_predicate_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inhabited_predicate_type in module {}", module_path!());
    };
}

mkfn!{
    inhabited_predicate_type_introspect!();
    # [doc = " N.B. this query should only be called through `Ty::inhabited_predicate`"] fn inhabited_predicate_type < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> InhabitedPredicate < 'tcx > { match * ty . kind () { Adt (adt , args) => tcx . inhabited_predicate_adt (adt . did ()) . instantiate (tcx , args) , Tuple (tys) => { InhabitedPredicate :: all (tcx , tys . iter () . map (| ty | ty . inhabited_predicate (tcx))) } Array (ty , len) => match len . try_to_target_usize (tcx) { Some (0) => InhabitedPredicate :: True , Some (1 ..) => ty . inhabited_predicate (tcx) , None => ty . inhabited_predicate (tcx) . or (tcx , InhabitedPredicate :: ConstIsZero (len)) , } , _ => bug ! ("unexpected TyKind, use `Ty::inhabited_predicate`") , } }
}