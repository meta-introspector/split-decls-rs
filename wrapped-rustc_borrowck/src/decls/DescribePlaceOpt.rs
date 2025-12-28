macro_rules! DescribePlaceOpt {
    () => {
        pub (super) struct DescribePlaceOpt { including_downcast : bool , # [doc = " Enable/Disable tuple fields."] # [doc = " For example `x` tuple. if it's `true` `x.0`. Otherwise `x`"] including_tuple_field : bool , }
    };
}

DescribePlaceOpt!();