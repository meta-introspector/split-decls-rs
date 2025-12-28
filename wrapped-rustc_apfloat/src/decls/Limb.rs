macro_rules! Limb {
    () => {
        # [doc = " Fundamental unit of big integer arithmetic, but also"] # [doc = " large to store the largest significands by itself."] type Limb = u128 ;
    };
}

Limb!();