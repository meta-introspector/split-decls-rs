macro_rules! AllocRef {
    () => {
        # [doc = " A reference to some allocation that was already bounds-checked for the given region"] # [doc = " and had the on-access machine hooks run."] # [derive (Copy , Clone)] pub struct AllocRef < 'a , 'tcx , Prov : Provenance , Extra , Bytes : AllocBytes = Box < [u8] > > { alloc : & 'a Allocation < Prov , Extra , Bytes > , range : AllocRange , tcx : TyCtxt < 'tcx > , alloc_id : AllocId , }
    };
}

AllocRef!()