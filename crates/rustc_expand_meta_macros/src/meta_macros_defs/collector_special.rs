#[macro_export]
macro_rules! CollectorSpecial {
    ($($p:tt)*) => { InvocationCollector<$($p)*> };
}