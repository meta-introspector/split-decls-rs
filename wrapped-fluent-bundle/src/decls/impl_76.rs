macro_rules! deps {
    () => {
        FluentNumberOptions!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl Default for FluentNumberOptions { fn default () -> Self { Self { r#type : Default :: default () , style : Default :: default () , currency : None , currency_display : Default :: default () , use_grouping : true , minimum_integer_digits : None , minimum_fraction_digits : None , maximum_fraction_digits : None , minimum_significant_digits : None , maximum_significant_digits : None , } } }
    };
}

impl_76!();