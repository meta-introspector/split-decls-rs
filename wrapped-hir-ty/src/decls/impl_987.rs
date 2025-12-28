macro_rules! deps {
    () => {
        TraitEnvironment!();
    };
}

macro_rules! impl_987 {
    () => {
        deps!();
        impl < 'db > TraitEnvironment < 'db > { pub fn empty (krate : Crate) -> Arc < Self > { Arc :: new (TraitEnvironment { krate , block : None , traits_from_clauses : Box :: default () , env : ParamEnv :: empty () , }) } pub fn new (krate : Crate , block : Option < BlockId > , traits_from_clauses : Box < [(Ty < 'db > , TraitId)] > , env : ParamEnv < 'db > ,) -> Arc < Self > { Arc :: new (TraitEnvironment { krate , block , traits_from_clauses , env }) } pub fn with_block (this : & mut Arc < Self > , block : BlockId) { Arc :: make_mut (this) . block = Some (block) ; } pub fn traits_in_scope_from_clauses (& self , ty : Ty < 'db >) -> impl Iterator < Item = TraitId > + '_ { self . traits_from_clauses . iter () . filter_map (move | (self_ty , trait_id) | (* self_ty == ty) . then_some (* trait_id)) } }
    };
}

impl_987!();