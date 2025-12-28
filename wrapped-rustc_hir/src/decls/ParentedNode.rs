macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! ParentedNode {
    () => {
        deps!();
        # [doc = " HIR node coupled with its parent's id in the same HIR owner."] # [doc = ""] # [doc = " The parent is trash when the node is a HIR owner."] # [derive (Clone , Copy , Debug)] pub struct ParentedNode < 'tcx > { pub parent : ItemLocalId , pub node : Node < 'tcx > , }
    };
}

ParentedNode!()