macro_rules! deps {
    () => {
        CallFrameInstructionIter!();
        Error!();
        Reader!();
        CallFrameInstruction!();
        Result!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        # [cfg (feature = "fallible-iterator")] impl < 'a , R : Reader > fallible_iterator :: FallibleIterator for CallFrameInstructionIter < 'a , R > { type Item = CallFrameInstruction < R :: Offset > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { CallFrameInstructionIter :: next (self) } }
    };
}

impl_247!();