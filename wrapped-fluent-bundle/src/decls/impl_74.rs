macro_rules! deps {
    () => {
        FluentNumberCurrencyDisplayStyle!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl From < & str > for FluentNumberCurrencyDisplayStyle { fn from (input : & str) -> Self { match input { "symbol" => Self :: Symbol , "code" => Self :: Code , "name" => Self :: Name , _ => Self :: default () , } } }
    };
}

impl_74!()