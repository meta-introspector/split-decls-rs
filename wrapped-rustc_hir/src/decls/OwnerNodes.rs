macro_rules! deps {
    () => {
        Body!();
        ParentedNode!();
    };
}

macro_rules! OwnerNodes {
    () => {
        deps!();
        # [doc = " Map of all HIR nodes inside the current owner."] # [doc = " These nodes are mapped by `ItemLocalId` alongside the index of their parent node."] # [doc = " The HIR tree, including bodies, is pre-hashed."] pub struct OwnerNodes < 'tcx > { # [doc = " Pre-computed hash of the full HIR. Used in the crate hash. Only present"] # [doc = " when incr. comp. is enabled."] pub opt_hash_including_bodies : Option < Fingerprint > , # [doc = " Full HIR for the current owner."] pub nodes : IndexVec < ItemLocalId , ParentedNode < 'tcx > > , # [doc = " Content of local bodies."] pub bodies : SortedMap < ItemLocalId , & 'tcx Body < 'tcx > > , }
    };
}

OwnerNodes!()