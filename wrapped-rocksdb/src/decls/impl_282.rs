macro_rules! deps {
    () => {
        MergeOperandsIter!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < 'a > Iterator for MergeOperandsIter < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < Self :: Item > { let operand = self . operands . get_operand (self . cursor) ? ; self . cursor += 1 ; Some (operand) } fn size_hint (& self) -> (usize , Option < usize >) { let remaining = self . operands . num_operands - self . cursor ; (remaining , Some (remaining)) } }
    };
}

impl_282!()