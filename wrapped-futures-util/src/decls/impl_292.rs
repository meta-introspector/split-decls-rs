macro_rules! impl_292 {
    () => {
        impl < St : Stream , FromA : Default , FromB : Default > Unzip < St , FromA , FromB > { fn finish (self : Pin < & mut Self >) -> (FromA , FromB) { let this = self . project () ; (mem :: take (this . left) , mem :: take (this . right)) } pub (super) fn new (stream : St) -> Self { Self { stream , left : Default :: default () , right : Default :: default () } } }
    };
}

impl_292!()