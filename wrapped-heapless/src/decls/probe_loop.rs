macro_rules! probe_loop {
    () => {
        macro_rules ! probe_loop { ($ probe_var : ident < $ len : expr , $ body : expr) => { loop { if $ probe_var < $ len { $ body $ probe_var += 1 ; } else { $ probe_var = 0 ; } } } }
    };
}

probe_loop!();