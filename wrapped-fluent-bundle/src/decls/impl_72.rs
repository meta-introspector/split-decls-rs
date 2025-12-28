macro_rules! deps {
    () => {
        FluentNumberStyle!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < & str > for FluentNumberStyle { fn from (input : & str) -> Self { match input { "decimal" => Self :: Decimal , "currency" => Self :: Currency , "percent" => Self :: Percent , _ => Self :: default () , } } }
    };
}

impl_72!();