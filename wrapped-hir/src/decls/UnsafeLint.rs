macro_rules! UnsafeLint {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum UnsafeLint { HardError , UnsafeOpInUnsafeFn , DeprecatedSafe2024 , }
    };
}

UnsafeLint!()