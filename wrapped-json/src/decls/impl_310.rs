macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < T : Into < Value > > FromIterator < T > for Value { # [doc = " Create a `Value::Array` by collecting an iterator of array elements."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let v = std::iter::repeat(42).take(5);"] # [doc = " let x: Value = v.collect();"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let v: Vec<_> = vec![\"lorem\", \"ipsum\", \"dolor\"];"] # [doc = " let x: Value = v.into_iter().collect();"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " use std::iter::FromIterator;"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let x: Value = Value::from_iter(vec![\"lorem\", \"ipsum\", \"dolor\"]);"] # [doc = " ```"] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { Value :: Array (iter . into_iter () . map (Into :: into) . collect ()) } }
    };
}

impl_310!();