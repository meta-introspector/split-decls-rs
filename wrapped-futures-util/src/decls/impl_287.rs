macro_rules! impl_287 {
    () => {
        impl < St : Stream , C : Default > Collect < St , C > { fn finish (self : Pin < & mut Self >) -> C { mem :: take (self . project () . collection) } pub (super) fn new (stream : St) -> Self { Self { stream , collection : Default :: default () } } }
    };
}

impl_287!()