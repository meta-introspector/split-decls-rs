macro_rules! other_47 {
    () => {
        macro pathvec_std ($ ($ rest : ident) ::+) { { vec ! [$ (sym ::$ rest) ,+] } }
    };
}

other_47!()