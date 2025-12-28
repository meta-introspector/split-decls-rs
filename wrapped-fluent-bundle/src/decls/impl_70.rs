macro_rules! deps {
    () => {
        FluentNumberType!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl From < & str > for FluentNumberType { fn from (input : & str) -> Self { match input { "cardinal" => Self :: Cardinal , "ordinal" => Self :: Ordinal , _ => Self :: default () , } } }
    };
}

impl_70!()