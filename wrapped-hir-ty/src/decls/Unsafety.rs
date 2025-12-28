macro_rules! Unsafety {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Unsafety { Safe , Unsafe , # [doc = " A lint."] DeprecatedSafe2024 , }
    };
}

Unsafety!();