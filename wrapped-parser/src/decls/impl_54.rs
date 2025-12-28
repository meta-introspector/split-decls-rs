macro_rules! deps {
    () => {
        Input!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [doc = " pub(crate) impl used by the parser to consume `Tokens`."] impl Input { pub (crate) fn kind (& self , idx : usize) -> SyntaxKind { self . kind . get (idx) . copied () . unwrap_or (SyntaxKind :: EOF) } pub (crate) fn contextual_kind (& self , idx : usize) -> SyntaxKind { self . contextual_kind . get (idx) . copied () . unwrap_or (SyntaxKind :: EOF) } pub (crate) fn is_joint (& self , n : usize) -> bool { let (idx , b_idx) = self . bit_index (n) ; self . joint [idx] & (1 << b_idx) != 0 } }
    };
}

impl_54!()