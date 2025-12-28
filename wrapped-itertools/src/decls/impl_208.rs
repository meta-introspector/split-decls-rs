macro_rules! deps {
    () => {
        PutBack!();
        Diff!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < I , J > Clone for Diff < I , J > where I : Iterator , J : Iterator , PutBack < I > : Clone , PutBack < J > : Clone , { fn clone (& self) -> Self { match self { Self :: FirstMismatch (idx , i , j) => Self :: FirstMismatch (* idx , i . clone () , j . clone ()) , Self :: Shorter (idx , i) => Self :: Shorter (* idx , i . clone ()) , Self :: Longer (idx , j) => Self :: Longer (* idx , j . clone ()) , } } }
    };
}

impl_208!()