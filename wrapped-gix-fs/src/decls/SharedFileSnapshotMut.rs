macro_rules! deps {
    () => {
        FileSnapshot!();
        SharedFileSnapshot!();
    };
}

macro_rules! SharedFileSnapshotMut {
    () => {
        deps!();
        # [doc = " Use this type for fields in structs that are to store the [`FileSnapshot`], typically behind an [`OwnShared`]."] # [doc = ""] # [doc = " Note that the resource itself is behind another [`OwnShared`] to allow it to be used without holding any kind of lock, hence"] # [doc = " without blocking updates while it is used."] # [derive (Debug , Default)] pub struct SharedFileSnapshotMut < T : std :: fmt :: Debug > (pub MutableOnDemand < Option < SharedFileSnapshot < T > > >) ;
    };
}

SharedFileSnapshotMut!()