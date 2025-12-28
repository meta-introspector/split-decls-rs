macro_rules! deps {
    () => {
        Ref!();
        InternalRef!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl From < InternalRef > for Ref { fn from (v : InternalRef) -> Self { match v { InternalRef :: Symbolic { path , target : Some (target) , tag , object , } => Ref :: Symbolic { full_ref_name : path , target , tag , object , } , InternalRef :: Symbolic { path , target : None , tag : None , object , } => Ref :: Direct { full_ref_name : path , object , } , InternalRef :: Symbolic { path , target : None , tag : Some (tag) , object , } => Ref :: Peeled { full_ref_name : path , tag , object , } , InternalRef :: Peeled { path , tag , object } => Ref :: Peeled { full_ref_name : path , tag , object , } , InternalRef :: Direct { path , object } => Ref :: Direct { full_ref_name : path , object , } , InternalRef :: SymbolicForLookup { .. } => { unreachable ! ("this case should have been removed during processing") } } } }
    };
}

impl_85!()