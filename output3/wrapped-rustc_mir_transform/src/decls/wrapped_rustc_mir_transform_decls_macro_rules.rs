use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// We import passes via this macro so that we can have a static list of pass names
/// (used to verify CLI arguments). It takes a list of modules, followed by the passes
/// declared within them.
/// ```ignore,macro-test
/// declare_passes! {
///     // Declare a single pass from the module `abort_unwinding_calls`
///     mod abort_unwinding_calls : AbortUnwindingCalls;
///     // When passes are grouped together as an enum, declare the two constituent passes
///     mod add_call_guards : AddCallGuards {
///         AllCallEdges,
///         CriticalCallEdges
///     };
///     // Declares multiple pass groups, each containing their own constituent passes
///     mod simplify : SimplifyCfg {
///         Initial,
///         /* omitted */
///     }, SimplifyLocals {
///         BeforeConstProp,
///         /* omitted */
///     };
/// }
/// ```
macro_rules! declare_passes {
    (
        $($vis:vis mod $mod_name:ident : $($pass_name:ident $({ $($ident:ident),* })?),+
        $(,)?;)*
    ) => {
        $($vis mod $mod_name; $(#[allow(unused_imports)] use $mod_name ::$pass_name as
        _;)+)* static PASS_NAMES : LazyLock < FxIndexSet <& str >> = LazyLock::new(||
        ["PreCodegen", $($(stringify!($pass_name), $($($mod_name ::$pass_name ::$ident
        .name(),)*)?)+)*].into_iter().collect());
    };
}
