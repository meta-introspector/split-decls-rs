macro_rules! glb {
    () => {
        fn glb (v1 : Variance , v2 : Variance) -> Variance { match (v1 , v2) { (Variance :: Invariant , _) | (_ , Variance :: Invariant) => Variance :: Invariant , (Variance :: Covariant , Variance :: Contravariant) => Variance :: Invariant , (Variance :: Contravariant , Variance :: Covariant) => Variance :: Invariant , (Variance :: Covariant , Variance :: Covariant) => Variance :: Covariant , (Variance :: Contravariant , Variance :: Contravariant) => Variance :: Contravariant , (x , Variance :: Bivariant) | (Variance :: Bivariant , x) => x , } }
    };
}

glb!()