macro_rules! deps {
    () => {
        FastPathRadix!();
        Limb!();
    };
}

macro_rules! add_temporary {
    () => {
        deps!();
        # [doc = " Add a temporary value to our mantissa."] macro_rules ! add_temporary { (@ mul $ result : ident , $ power : expr , $ value : expr) => { $ result . data . mul_small ($ power) . unwrap () ; $ result . data . add_small ($ value) . unwrap () ; } ; ($ format : ident , $ result : ident , $ counter : ident , $ value : ident) => { if $ counter != 0 { let small_power = unsafe { int_pow_fast_path ($ counter , FastPathRadix :: Ten) } ; add_temporary ! (@ mul $ result , small_power as Limb , $ value) ; $ counter = 0 ; $ value = 0 ; } } ; (@ end $ format : ident , $ result : ident , $ counter : ident , $ value : ident) => { if $ counter != 0 { let small_power = unsafe { int_pow_fast_path ($ counter , FastPathRadix :: Ten) } ; add_temporary ! (@ mul $ result , small_power as Limb , $ value) ; } } ; (@ max $ format : ident , $ result : ident , $ counter : ident , $ value : ident , $ max : ident) => { add_temporary ! (@ mul $ result , $ max , $ value) ; $ counter = 0 ; $ value = 0 ; } ; }
    };
}

add_temporary!()