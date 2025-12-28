macro_rules! deps {
    () => {
        InlineAsmTemplatePiece!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl fmt :: Display for InlineAsmTemplatePiece { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: String (s) => { for c in s . chars () { match c { '{' => f . write_str ("{{") ? , '}' => f . write_str ("}}") ? , _ => c . fmt (f) ? , } } Ok (()) } Self :: Placeholder { operand_idx , modifier : Some (modifier) , .. } => { write ! (f , "{{{operand_idx}:{modifier}}}") } Self :: Placeholder { operand_idx , modifier : None , .. } => { write ! (f , "{{{operand_idx}}}") } } } }
    };
}

impl_148!();