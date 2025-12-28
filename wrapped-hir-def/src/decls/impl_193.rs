macro_rules! deps {
    () => {
        UseTreeKind!();
        UseTree!();
        ImportAlias!();
        ImportKind!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl UseTree { # [doc = " The [`UseTreeKind`] of this `UseTree`."] pub fn kind (& self) -> & UseTreeKind { & self . kind } fn expand_impl (& self , prefix : Option < ModPath > , counting_index : & mut u32 , cb : & mut impl FnMut (Idx < ast :: UseTree > , ModPath , ImportKind , Option < ImportAlias >) ,) { fn concat_mod_paths (prefix : Option < ModPath > , path : & ModPath ,) -> Option < (ModPath , ImportKind) > { match (prefix , path . kind) { (None , _) => Some ((path . clone () , ImportKind :: Plain)) , (Some (mut prefix) , PathKind :: Plain) => { prefix . extend (path . segments () . iter () . cloned ()) ; Some ((prefix , ImportKind :: Plain)) } (Some (mut prefix) , PathKind :: Super (n)) if n > 0 && prefix . segments () . is_empty () => { match & mut prefix . kind { PathKind :: Super (m) => { cov_mark :: hit ! (concat_super_mod_paths) ; * m += n ; prefix . extend (path . segments () . iter () . cloned ()) ; Some ((prefix , ImportKind :: Plain)) } _ => None , } } (Some (prefix) , PathKind :: SELF) if path . segments () . is_empty () => { Some ((prefix , ImportKind :: TypeOnly)) } (Some (_) , _) => None , } } match & self . kind { UseTreeKind :: Single { path , alias } => { if let Some ((path , kind)) = concat_mod_paths (prefix , path) { cb (Idx :: from_raw (RawIdx :: from_u32 (* counting_index)) , path , kind , alias . clone ()) ; } } UseTreeKind :: Glob { path : Some (path) } => { if let Some ((path , _)) = concat_mod_paths (prefix , path) { cb (Idx :: from_raw (RawIdx :: from_u32 (* counting_index)) , path , ImportKind :: Glob , None ,) ; } } UseTreeKind :: Glob { path : None } => { if let Some (prefix) = prefix { cb (Idx :: from_raw (RawIdx :: from_u32 (* counting_index)) , prefix , ImportKind :: Glob , None ,) ; } } UseTreeKind :: Prefixed { prefix : additional_prefix , list } => { let prefix = match additional_prefix { Some (path) => match concat_mod_paths (prefix , path) { Some ((path , ImportKind :: Plain)) => Some (path) , _ => return , } , None => prefix , } ; for tree in & * * list { * counting_index += 1 ; tree . expand_impl (prefix . clone () , counting_index , cb) ; } } } } }
    };
}

impl_193!();