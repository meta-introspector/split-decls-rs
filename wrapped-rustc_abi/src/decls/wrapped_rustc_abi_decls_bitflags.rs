use serde::{Deserialize, Serialize};
use std::collections::HashMap;
bitflags! {
    impl ReprFlags : u8 { const IS_C = 1 << 0; const IS_SIMD = 1 << 1; const
    IS_TRANSPARENT = 1 << 2; #[doc =
    " Internal only for now. If true, don't reorder fields."] #[doc =
    " On its own it does not prevent ABI optimizations."] const IS_LINEAR = 1 << 3; #[doc
    = " If true, the type's crate has opted into layout randomization."] #[doc =
    " Other flags can still inhibit reordering and thus randomization."] #[doc =
    " The seed stored in `ReprOptions.field_shuffle_seed`."] const RANDOMIZE_LAYOUT = 1
    << 4; #[doc = " If true, the type is always passed indirectly by non-Rustic ABIs."]
    #[doc = " See [`TyAndLayout::pass_indirectly_in_non_rustic_abis`] for details."]
    const PASS_INDIRECTLY_IN_NON_RUSTIC_ABIS = 1 << 5; #[doc =
    " Any of these flags being set prevent field reordering optimisation."] const
    FIELD_ORDER_UNOPTIMIZABLE = ReprFlags::IS_C.bits() | ReprFlags::IS_SIMD.bits() |
    ReprFlags::IS_LINEAR.bits(); const ABI_UNOPTIMIZABLE = ReprFlags::IS_C.bits() |
    ReprFlags::IS_SIMD.bits(); }
}
