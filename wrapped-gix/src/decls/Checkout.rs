macro_rules! deps {
    () => {
        Default!();
        Clone!();
    };
}

macro_rules! Checkout {
    () => {
        deps!();
        # [doc = " The `checkout` top-level section."] # [derive (Copy , Clone , Default)] pub struct Checkout ;
    };
}

Checkout!()