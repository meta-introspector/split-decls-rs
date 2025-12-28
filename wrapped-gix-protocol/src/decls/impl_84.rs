macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Ref { # [doc = " Provide shared fields referring to the ref itself, namely `(name, target, [peeled])`."] # [doc = " In case of peeled refs, the tag object itself is returned as it is what the ref directly refers to, and target of the tag is returned"] # [doc = " as `peeled`."] # [doc = " If `unborn`, the first object id will be the null oid."] pub fn unpack (& self) -> (& BStr , Option < & gix_hash :: oid > , Option < & gix_hash :: oid >) { match self { Ref :: Direct { full_ref_name , object } => (full_ref_name . as_ref () , Some (object) , None) , Ref :: Symbolic { full_ref_name , tag , object , .. } => (full_ref_name . as_ref () , Some (tag . as_deref () . unwrap_or (object)) , tag . as_deref () . map (| _ | object . as_ref ()) ,) , Ref :: Peeled { full_ref_name , tag : object , object : peeled , } => (full_ref_name . as_ref () , Some (object) , Some (peeled)) , Ref :: Unborn { full_ref_name , target : _ , } => (full_ref_name . as_ref () , None , None) , } } }
    };
}

impl_84!()