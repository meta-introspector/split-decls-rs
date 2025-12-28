macro_rules! deps {
    () => {
        TypeOrConst!();
    };
}

macro_rules! AstSubsts {
    () => {
        deps!();
        # [derive (Default , Debug)] struct AstSubsts { types_and_consts : Vec < TypeOrConst > , lifetimes : Vec < ast :: LifetimeArg > , }
    };
}

AstSubsts!()