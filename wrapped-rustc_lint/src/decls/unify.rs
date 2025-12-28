macro_rules! unify {
    () => {
        # [doc = " What is the variance that satisfies the two variances?"] fn unify (a : ty :: Variance , b : ty :: Variance) -> ty :: Variance { match (a , b) { (ty :: Bivariant , other) | (other , ty :: Bivariant) => other , (ty :: Invariant , _) | (_ , ty :: Invariant) => ty :: Invariant , (ty :: Contravariant , ty :: Covariant) | (ty :: Covariant , ty :: Contravariant) => ty :: Invariant , (ty :: Contravariant , ty :: Contravariant) => ty :: Contravariant , (ty :: Covariant , ty :: Covariant) => ty :: Covariant , } }
    };
}

unify!();