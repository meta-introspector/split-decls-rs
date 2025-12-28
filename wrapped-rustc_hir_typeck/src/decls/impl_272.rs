macro_rules! deps {
    () => {
        Pick!();
        Candidate!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < 'tcx > Candidate < 'tcx > { fn to_unadjusted_pick (& self , self_ty : Ty < 'tcx > , unstable_candidates : Vec < (Candidate < 'tcx > , Symbol) > ,) -> Pick < 'tcx > { Pick { item : self . item , kind : match self . kind { InherentImplCandidate { .. } => InherentImplPick , ObjectCandidate (_) => ObjectPick , TraitCandidate (_) => TraitPick , WhereClauseCandidate (trait_ref) => { assert ! (! trait_ref . skip_binder () . args . has_infer () && ! trait_ref . skip_binder () . args . has_placeholders ()) ; WhereClausePick (trait_ref) } } , import_ids : self . import_ids . clone () , autoderefs : 0 , autoref_or_ptr_adjustment : None , self_ty , unstable_candidates , receiver_steps : match self . kind { InherentImplCandidate { receiver_steps , .. } => Some (receiver_steps) , _ => None , } , shadowed_candidates : vec ! [] , } } }
    };
}

impl_272!()