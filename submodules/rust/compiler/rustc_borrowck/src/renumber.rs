mkuse!{use rustc_index :: IndexSlice ;}
mkuse!{use rustc_infer :: infer :: NllRegionVariableOrigin ;}
mkuse!{use rustc_middle :: mir :: visit :: { MutVisitor , TyContext } ;}
mkuse!{use rustc_middle :: mir :: { Body , ConstOperand , Location , Promoted } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgsRef , Ty , TyCtxt , TypeFoldable , fold_regions } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: BorrowckInferCtxt ;}

macro_rules! renumber_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function renumber_mir in module {}", module_path!());
    };
}

mkfn!{
    renumber_mir_introspect!();
    # [doc = " Replaces all free regions appearing in the MIR with fresh"] # [doc = " inference variables, returning the number of variables created."] # [instrument (skip (infcx , body , promoted) , level = "debug")] pub (crate) fn renumber_mir < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & mut Body < 'tcx > , promoted : & mut IndexSlice < Promoted , Body < 'tcx > > ,) { debug ! (? body . arg_count) ; let mut renumberer = RegionRenumberer { infcx } ; for body in promoted . iter_mut () { renumberer . visit_body (body) ; } renumberer . visit_body (body) ; }
}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , Eq , PartialEq , Hash)] pub (crate) enum RegionCtxt { Location (Location) , TyContext (TyContext) , Free (Symbol) , LateBound (Symbol) , Existential (Option < Symbol >) , Placeholder (Symbol) , Unknown , }}}
mkitem!{mkimpl!{impl RegionCtxt { # [doc = " Used to determine the representative of a component in the strongly connected"] # [doc = " constraint graph"] pub (crate) fn preference_value (self) -> usize { match self { RegionCtxt :: Unknown => 1 , RegionCtxt :: Existential (None) => 2 , RegionCtxt :: Existential (Some (_)) | RegionCtxt :: Free (_) => 2 , RegionCtxt :: Location (_) => 3 , RegionCtxt :: TyContext (_) => 4 , _ => 5 , } } }}}
mkitem!{mkstruct!{struct RegionRenumberer < 'a , 'tcx > { infcx : & 'a BorrowckInferCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > RegionRenumberer < 'a , 'tcx > { # [doc = " Replaces all regions appearing in `value` with fresh inference"] # [doc = " variables."] fn renumber_regions < T , F > (& mut self , value : T , region_ctxt_fn : F) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , F : Fn () -> RegionCtxt , { let origin = NllRegionVariableOrigin :: Existential { name : None } ; fold_regions (self . infcx . tcx , value , | _region , _depth | { self . infcx . next_nll_region_var (origin , | | region_ctxt_fn ()) }) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > MutVisitor < 'tcx > for RegionRenumberer < 'a , 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } # [instrument (skip (self) , level = "debug")] fn visit_ty (& mut self , ty : & mut Ty < 'tcx > , ty_context : TyContext) { if matches ! (ty_context , TyContext :: ReturnTy (_)) { return ; } * ty = self . renumber_regions (* ty , | | RegionCtxt :: TyContext (ty_context)) ; debug ! (? ty) ; } # [instrument (skip (self) , level = "debug")] fn visit_args (& mut self , args : & mut GenericArgsRef < 'tcx > , location : Location) { * args = self . renumber_regions (* args , | | RegionCtxt :: Location (location)) ; debug ! (? args) ; } # [instrument (skip (self) , level = "debug")] fn visit_region (& mut self , region : & mut ty :: Region < 'tcx > , location : Location) { let old_region = * region ; * region = self . renumber_regions (old_region , | | RegionCtxt :: Location (location)) ; debug ! (? region) ; } # [instrument (skip (self) , level = "debug")] fn visit_ty_const (& mut self , ct : & mut ty :: Const < 'tcx > , location : Location) { let old_ct = * ct ; * ct = self . renumber_regions (old_ct , | | RegionCtxt :: Location (location)) ; debug ! (? ct) ; } # [instrument (skip (self) , level = "debug")] fn visit_const_operand (& mut self , constant : & mut ConstOperand < 'tcx > , location : Location) { let const_ = constant . const_ ; constant . const_ = self . renumber_regions (const_ , | | RegionCtxt :: Location (location)) ; debug ! ("constant: {:#?}" , constant) ; } }}}