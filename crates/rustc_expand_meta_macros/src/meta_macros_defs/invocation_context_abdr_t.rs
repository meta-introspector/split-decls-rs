#[macro_export]
macro_rules! InvocationContextABDRT {
    () => { InvocationCollector<'a, 'b, DRT> };
}

