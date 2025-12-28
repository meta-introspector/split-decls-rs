macro_rules! Warnings {
    () => {
        # [doc = " Contains warnings collected during code generation."] # [derive (Debug)] pub struct Warnings (Vec < String >) ;
    };
}

Warnings!();