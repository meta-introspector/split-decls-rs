macro_rules! __bitflags_match {
    () => {
        # [doc = " Expand the `bitflags_match` macro"] # [macro_export] # [doc (hidden)] macro_rules ! __bitflags_match { ($ operation : expr , { $ pattern : expr => { $ ($ body : tt) * } , $ ($ t : tt) + }) => { $ crate :: __bitflags_match ! ($ operation , { $ pattern => { $ ($ body) * } $ ($ t) + }) } ; ($ operation : expr , { $ pattern : expr => { $ ($ body : tt) * } $ ($ t : tt) + }) => { { if $ operation == $ pattern { return { $ ($ body) * } ; } $ crate :: __bitflags_match ! ($ operation , { $ ($ t) + }) } } ; ($ operation : expr , { $ pattern : expr => $ body : expr , $ ($ t : tt) + }) => { { if $ operation == $ pattern { return $ body ; } $ crate :: __bitflags_match ! ($ operation , { $ ($ t) + }) } } ; ($ operation : expr , { _ => $ default : expr $ (,) ? }) => { $ default } }
    };
}

__bitflags_match!();