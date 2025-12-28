macro_rules! deps {
    () => {
        PikeVM!();
        PikeVMCache!();
        RegexInfo!();
        BuildError!();
        Prefilter!();
        PikeVMEngine!();
        NFA!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl PikeVM { pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA ,) -> Result < PikeVM , BuildError > { PikeVMEngine :: new (info , pre , nfa) . map (PikeVM) } pub (crate) fn create_cache (& self) -> PikeVMCache { PikeVMCache :: none () } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self) -> & PikeVMEngine { & self . 0 } }
    };
}

impl_398!();