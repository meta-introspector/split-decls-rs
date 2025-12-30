// Generated macro for macro_154 (macro)
macro_rules! Depcrate_runtimemacro_154 {
() => {
// Module: crate::runtime
// Provides: {"macro_154"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [doc = " A helper for constructing [`UdpSender`]s from an underlying `Socket` type."] # [doc = ""] # [doc = " This struct implements [`UdpSender`] if `MakeWritableFn` produces a `WritableFut`."] # [doc = ""] # [doc = " Also serves as a trick, since `WritableFut` doesn't need to be a named future,"] # [doc = " it can be an anonymous async block, as long as `MakeWritableFn` produces that"] # [doc = " anonymous async block type."] # [doc = ""] # [doc = " The `UdpSenderHelper` generic type parameters don't need to named, as it will be"] # [doc = " used in its dyn-compatible form as a `Pin<Box<dyn UdpSender>>`."] struct UdpSenderHelper < Socket , MakeWritableFutFn , WritableFut > { socket : Socket , make_writable_fut_fn : MakeWritableFutFn , # [pin] writable_fut : Option < WritableFut >, } }
};
}
