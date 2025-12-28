macro_rules! signed_short {
    () => {
        fn signed_short (n : f64) -> String { let n_abs = n . abs () ; let sign = if n >= 0.0 { '+' } else { '\u{2212}' } ; if n_abs < 10.0 { format ! ("{}{:.4}" , sign , n_abs) } else if n_abs < 100.0 { format ! ("{}{:.3}" , sign , n_abs) } else if n_abs < 1000.0 { format ! ("{}{:.2}" , sign , n_abs) } else if n_abs < 10000.0 { format ! ("{}{:.1}" , sign , n_abs) } else { format ! ("{}{:.0}" , sign , n_abs) } }
    };
}

signed_short!()