macro_rules! comp_r {
    () => {
        fn comp_r (z : f64) -> f64 { let p = z * (P_S0 + z * (P_S1 + z * (P_S2 + z * (P_S3 + z * (P_S4 + z * P_S5))))) ; let q = 1.0 + z * (Q_S1 + z * (Q_S2 + z * (Q_S3 + z * Q_S4))) ; p / q }
    };
}

comp_r!()