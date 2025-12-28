macro_rules! scalar_unit {
    () => {
        fn scalar_unit (dl : & TargetDataLayout , value : Primitive) -> Scalar { Scalar :: Initialized { value , valid_range : WrappingRange :: full (value . size (dl)) } }
    };
}

scalar_unit!();