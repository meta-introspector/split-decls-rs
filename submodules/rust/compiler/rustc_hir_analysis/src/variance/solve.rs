mkuse!{use rustc_hir :: def_id :: DefIdMap ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: constraints :: * ;}
mkuse!{use super :: terms :: VarianceTerm :: * ;}
mkuse!{use super :: terms :: * ;}

macro_rules! glb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function glb in module {}", module_path!());
    };
}

mkfn!{
    glb_introspect!();
    fn glb (v1 : ty :: Variance , v2 : ty :: Variance) -> ty :: Variance { match (v1 , v2) { (ty :: Invariant , _) | (_ , ty :: Invariant) => ty :: Invariant , (ty :: Covariant , ty :: Contravariant) => ty :: Invariant , (ty :: Contravariant , ty :: Covariant) => ty :: Invariant , (ty :: Covariant , ty :: Covariant) => ty :: Covariant , (ty :: Contravariant , ty :: Contravariant) => ty :: Contravariant , (x , ty :: Bivariant) | (ty :: Bivariant , x) => x , } }
}
mkitem!{mkstruct!{struct SolveContext < 'a , 'tcx > { terms_cx : TermsContext < 'a , 'tcx > , constraints : Vec < Constraint < 'a > > , solutions : Vec < ty :: Variance > , }}}

macro_rules! solve_constraints_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function solve_constraints in module {}", module_path!());
    };
}

mkfn!{
    solve_constraints_introspect!();
    pub (crate) fn solve_constraints < 'tcx > (constraints_cx : ConstraintContext < '_ , 'tcx > ,) -> ty :: CrateVariancesMap < 'tcx > { let ConstraintContext { terms_cx , constraints , .. } = constraints_cx ; let mut solutions = vec ! [ty :: Bivariant ; terms_cx . inferred_terms . len ()] ; for (id , variances) in & terms_cx . lang_items { let InferredIndex (start) = terms_cx . inferred_starts [id] ; for (i , & variance) in variances . iter () . enumerate () { solutions [start + i] = variance ; } } let mut solutions_cx = SolveContext { terms_cx , constraints , solutions } ; solutions_cx . solve () ; let variances = solutions_cx . create_map () ; ty :: CrateVariancesMap { variances } }
}
mkitem!{mkimpl!{impl < 'a , 'tcx > SolveContext < 'a , 'tcx > { fn solve (& mut self) { let mut changed = true ; while changed { changed = false ; for constraint in & self . constraints { let Constraint { inferred , variance : term } = * constraint ; let InferredIndex (inferred) = inferred ; let variance = self . evaluate (term) ; let old_value = self . solutions [inferred] ; let new_value = glb (variance , old_value) ; if old_value != new_value { debug ! ("updating inferred {} \
                            from {:?} to {:?} due to {:?}" , inferred , old_value , new_value , term) ; self . solutions [inferred] = new_value ; changed = true ; } } } } fn enforce_const_invariance (& self , generics : & ty :: Generics , variances : & mut [ty :: Variance]) { let tcx = self . terms_cx . tcx ; for param in generics . own_params . iter () { if let ty :: GenericParamDefKind :: Const { .. } = param . kind { variances [param . index as usize] = ty :: Invariant ; } } if let Some (def_id) = generics . parent { self . enforce_const_invariance (tcx . generics_of (def_id) , variances) ; } } fn create_map (& self) -> DefIdMap < & 'tcx [ty :: Variance] > { let tcx = self . terms_cx . tcx ; let solutions = & self . solutions ; DefIdMap :: from (self . terms_cx . inferred_starts . items () . map (| (& def_id , & InferredIndex (start)) | { let generics = tcx . generics_of (def_id) ; let count = generics . count () ; let variances = tcx . arena . alloc_slice (& solutions [start .. (start + count)]) ; self . enforce_const_invariance (generics , variances) ; if let ty :: FnDef (..) = tcx . type_of (def_id) . instantiate_identity () . kind () { for variance in variances . iter_mut () { if * variance == ty :: Bivariant { * variance = ty :: Invariant ; } } } (def_id . to_def_id () , & * variances) } ,)) } fn evaluate (& self , term : VarianceTermPtr < 'a >) -> ty :: Variance { match * term { ConstantTerm (v) => v , TransformTerm (t1 , t2) => { let v1 = self . evaluate (t1) ; let v2 = self . evaluate (t2) ; v1 . xform (v2) } InferredTerm (InferredIndex (index)) => self . solutions [index] , } } }}}