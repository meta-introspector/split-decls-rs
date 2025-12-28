macro_rules! deps {
    () => {
        Directive!();
        MetaDirectiveInvocation!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl From < Directive > for MetaDirectiveInvocation { fn from (directive : Directive) -> Self { Self { name : directive . name , args : directive . args , } } }
    };
}

impl_416!();