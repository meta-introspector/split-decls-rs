macro_rules! deps {
    () => {
        ConstChoice!();
    };
}

macro_rules! ConstCtOption {
    () => {
        deps!();
        # [doc = " An equivalent of `subtle::CtOption` usable in a `const fn` context."] # [derive (Debug , Clone)] pub struct ConstCtOption < T > { value : T , is_some : ConstChoice , }
    };
}

ConstCtOption!()