macro_rules! deps {
    () => {
        FrameIter!();
        Result!();
        Error!();
        Frame!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < 'ctx , R > fallible_iterator :: FallibleIterator for FrameIter < 'ctx , R > where R : gimli :: Reader + 'ctx , { type Item = Frame < 'ctx , R > ; type Error = Error ; # [inline] fn next (& mut self) -> Result < Option < Frame < 'ctx , R > > , Error > { self . next () } }
    };
}

impl_13!()