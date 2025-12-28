macro_rules! deps {
    () => {
        InterpCx!();
        Machine!();
    };
}

macro_rules! DumpAllocs {
    () => {
        deps!();
        # [doc (hidden)] # [doc = " There's no way to use this directly, it's just a helper struct for the `dump_alloc(s)` methods."] pub struct DumpAllocs < 'a , 'tcx , M : Machine < 'tcx > > { ecx : & 'a InterpCx < 'tcx , M > , allocs : Vec < AllocId > , }
    };
}

DumpAllocs!()