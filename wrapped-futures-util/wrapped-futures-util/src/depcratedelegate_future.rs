// Generated macro for delegate_future (macro)
macro_rules! Depcratedelegate_future {
() => {
// Module: crate
// Provides: {"delegate_future"}
// Dependencies: {}
macro_rules ! delegate_future { ($ field : ident) => { fn poll (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >,) -> core :: task :: Poll < Self :: Output > { self . project () .$ field . poll (cx) } } ; }
};
}
