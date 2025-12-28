macro_rules! Namespace {
    () => {
        # [derive (PartialEq , Eq , Hash , Copy , Clone , Debug)] pub enum Namespace { Types , Values , Macros , }
    };
}

Namespace!();