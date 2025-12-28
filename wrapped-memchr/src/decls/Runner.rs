macro_rules! Runner {
    () => {
        # [doc = " Runs a host of substring search tests."] # [doc = ""] # [doc = " This has support for \"partial\" substring search implementations only work"] # [doc = " for a subset of needles/haystacks. For example, the \"packed pair\" substring"] # [doc = " search implementation only works for haystacks of some minimum length based"] # [doc = " of the pair of bytes selected and the size of the vector used."] pub (crate) struct Runner { fwd : Option < Box < dyn FnMut (& [u8] , & [u8]) -> Option < Option < usize > > + 'static > , > , rev : Option < Box < dyn FnMut (& [u8] , & [u8]) -> Option < Option < usize > > + 'static > , > , }
    };
}

Runner!()