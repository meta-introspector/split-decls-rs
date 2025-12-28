macro_rules! deps {
    () => {
        AmbigArg!();
        TyKind!();
        Res!();
        DefKind!();
        ConstArgKind!();
        GenericArg!();
        Visitor!();
        QPath!();
        MutTy!();
        Path!();
        Ty!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < 'hir > Ty < 'hir > { pub fn peel_refs (& self) -> & Self { let mut final_ty = self ; while let TyKind :: Ref (_ , MutTy { ty , .. }) = & final_ty . kind { final_ty = ty ; } final_ty } # [doc = " Returns `true` if `param_def_id` matches the `bounded_ty` of this predicate."] pub fn as_generic_param (& self) -> Option < (DefId , Ident) > { let TyKind :: Path (QPath :: Resolved (None , path)) = self . kind else { return None ; } ; let [segment] = & path . segments else { return None ; } ; match path . res { Res :: Def (DefKind :: TyParam , def_id) | Res :: SelfTyParam { trait_ : def_id } => { Some ((def_id , segment . ident)) } _ => None , } } pub fn find_self_aliases (& self) -> Vec < Span > { use crate :: intravisit :: Visitor ; struct MyVisitor (Vec < Span >) ; impl < 'v > Visitor < 'v > for MyVisitor { fn visit_ty (& mut self , t : & 'v Ty < 'v , AmbigArg >) { if matches ! (& t . kind , TyKind :: Path (QPath :: Resolved (_ , Path { res : crate :: def :: Res :: SelfTyAlias { .. } , .. } ,))) { self . 0 . push (t . span) ; return ; } crate :: intravisit :: walk_ty (self , t) ; } } let mut my_visitor = MyVisitor (vec ! []) ; my_visitor . visit_ty_unambig (self) ; my_visitor . 0 } # [doc = " Whether `ty` is a type with `_` placeholders that can be inferred. Used in diagnostics only to"] # [doc = " use inference to provide suggestions for the appropriate type if possible."] pub fn is_suggestable_infer_ty (& self) -> bool { fn are_suggestable_generic_args (generic_args : & [GenericArg < '_ >]) -> bool { generic_args . iter () . any (| arg | match arg { GenericArg :: Type (ty) => ty . as_unambig_ty () . is_suggestable_infer_ty () , GenericArg :: Infer (_) => true , _ => false , }) } debug ! (? self) ; match & self . kind { TyKind :: Infer (()) => true , TyKind :: Slice (ty) => ty . is_suggestable_infer_ty () , TyKind :: Array (ty , length) => { ty . is_suggestable_infer_ty () || matches ! (length . kind , ConstArgKind :: Infer (..)) } TyKind :: Tup (tys) => tys . iter () . any (Self :: is_suggestable_infer_ty) , TyKind :: Ptr (mut_ty) | TyKind :: Ref (_ , mut_ty) => mut_ty . ty . is_suggestable_infer_ty () , TyKind :: Path (QPath :: TypeRelative (ty , segment)) => { ty . is_suggestable_infer_ty () || are_suggestable_generic_args (segment . args () . args) } TyKind :: Path (QPath :: Resolved (ty_opt , Path { segments , .. })) => { ty_opt . is_some_and (Self :: is_suggestable_infer_ty) || segments . iter () . any (| segment | are_suggestable_generic_args (segment . args () . args)) } _ => false , } } }
    };
}

impl_267!()