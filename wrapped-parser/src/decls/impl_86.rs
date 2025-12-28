macro_rules! impl_86 {
    () => {
        impl From < u16 > for SyntaxKind { # [inline] fn from (d : u16) -> SyntaxKind { assert ! (d <= (SyntaxKind :: __LAST as u16)) ; unsafe { std :: mem :: transmute :: < u16 , SyntaxKind > (d) } } }
    };
}

impl_86!();