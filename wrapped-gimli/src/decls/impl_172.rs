macro_rules! deps {
    () => {
        Reader!();
        Result!();
        Error!();
        Pointer!();
        EhHdrTableIter!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < 'a , 'bases , R : Reader > fallible_iterator :: FallibleIterator for EhHdrTableIter < 'a , 'bases , R > { type Item = (Pointer , Pointer) ; type Error = Error ; fn next (& mut self) -> Result < Option < Self :: Item > > { EhHdrTableIter :: next (self) } fn size_hint (& self) -> (usize , Option < usize >) { use core :: convert :: TryInto ; (self . remain . try_into () . unwrap_or (0) , self . remain . try_into () . ok () ,) } fn nth (& mut self , n : usize) -> Result < Option < Self :: Item > > { EhHdrTableIter :: nth (self , n) } }
    };
}

impl_172!()