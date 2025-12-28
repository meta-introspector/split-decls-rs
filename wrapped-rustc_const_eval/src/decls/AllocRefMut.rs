macro_rules! AllocRefMut {
    () => {
        # [doc = " A reference to some allocation that was already bounds-checked for the given region"] # [doc = " and had the on-access machine hooks run."] pub struct AllocRefMut < 'a , 'tcx , Prov : Provenance , Extra , Bytes : AllocBytes = Box < [u8] > > { alloc : & 'a mut Allocation < Prov , Extra , Bytes > , range : AllocRange , tcx : TyCtxt < 'tcx > , alloc_id : AllocId , }
    };
}

AllocRefMut!();