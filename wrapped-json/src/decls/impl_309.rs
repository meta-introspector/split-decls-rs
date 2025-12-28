macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < T : Clone + Into < Value > > From < & [T] > for Value { # [doc = " Convert a slice to `Value::Array`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde_json::Value;"] # [doc = ""] # [doc = " let v: &[&str] = &[\"lorem\", \"ipsum\", \"dolor\"];"] # [doc = " let x: Value = v.into();"] # [doc = " ```"] fn from (f : & [T]) -> Self { Value :: Array (f . iter () . cloned () . map (Into :: into) . collect ()) } }
    };
}

impl_309!();